#!/usr/bin/env python3
"""Run the Round 2 TeaQL modeling parameter matrix."""

from __future__ import annotations

import argparse
import json
import time
import urllib.error
import urllib.request
from pathlib import Path

from validate_ksml import validate_text


MATRIX = [
    {
        "id": "no-think-greedy",
        "temperature": 0.0,
        "top_p": 1.0,
        "max_tokens": 7000,
        "chat_template_kwargs": {"enable_thinking": False},
    },
    {
        "id": "no-think-official-sampling",
        "temperature": 1.0,
        "top_p": 0.95,
        "max_tokens": 7000,
        "chat_template_kwargs": {"enable_thinking": False},
    },
    {
        "id": "no-think-low-temperature",
        "temperature": 0.2,
        "top_p": 0.9,
        "max_tokens": 7000,
        "chat_template_kwargs": {"enable_thinking": False},
    },
    {
        "id": "think-budget-1024",
        "temperature": 1.0,
        "top_p": 0.95,
        "max_tokens": 8000,
        "chat_template_kwargs": {
            "enable_thinking": True,
            "reasoning_budget": 1024,
        },
    },
    {
        "id": "repair-official-feedback",
        "temperature": 1.0,
        "top_p": 0.95,
        "max_tokens": 7000,
        "chat_template_kwargs": {"enable_thinking": False},
        "repair_from": "no-think-official-sampling",
    },
]


def post_json(url: str, payload: dict, timeout: int) -> dict:
    request = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return json.loads(response.read().decode("utf-8"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--model", default="nemotron-3-super")
    parser.add_argument("--rules-file", required=True, type=Path)
    parser.add_argument(
        "--task-file",
        type=Path,
        default=Path("benchmarks/round-2/school-platform-task.txt"),
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("artifacts/round-2/modeling-runs"),
    )
    parser.add_argument("--only", choices=[item["id"] for item in MATRIX])
    parser.add_argument("--timeout", type=int, default=900)
    args = parser.parse_args()

    rules = args.rules_file.read_text(encoding="utf-8")
    task = args.task_file.read_text(encoding="utf-8")
    args.output_dir.mkdir(parents=True, exist_ok=True)
    selected = [item for item in MATRIX if not args.only or item["id"] == args.only]
    summaries = []

    for config in selected:
        system = (
            "You are a TeaQL KSML modeling agent. Follow the supplied rules exactly. "
            "Return XML only. The server separates reasoning_content from final content."
        )
        if not config["chat_template_kwargs"]["enable_thinking"]:
            system = "/no_think " + system
        messages = [
            {"role": "system", "content": system},
            {
                "role": "user",
                "content": f"{task}\n\n# TeaQL KSML rules\n\n{rules}",
            },
        ]
        repair_from = config.get("repair_from")
        if repair_from:
            prior_path = args.output_dir / f"{repair_from}.json"
            prior = json.loads(prior_path.read_text(encoding="utf-8"))
            messages.extend(
                [
                    {"role": "assistant", "content": prior["content"]},
                    {
                        "role": "user",
                        "content": (
                            "The deterministic validator rejected that XML with "
                            "these errors:\n- "
                            + "\n- ".join(prior["validation"]["errors"])
                            + "\nReturn the complete corrected XML only. Preserve "
                            "all valid objects and relationships, remove or rename "
                            "the offending fields, and re-check XML syntax."
                        ),
                    },
                ]
            )
        payload = {
            "model": args.model,
            "messages": messages,
            "temperature": config["temperature"],
            "top_p": config["top_p"],
            "max_tokens": config["max_tokens"],
            "chat_template_kwargs": config["chat_template_kwargs"],
        }

        started = time.monotonic()
        error = None
        response = None
        try:
            response = post_json(
                args.base_url.rstrip("/") + "/v1/chat/completions",
                payload,
                args.timeout,
            )
        except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as exc:
            error = f"{type(exc).__name__}: {exc}"
        duration = time.monotonic() - started

        content = ""
        reasoning_content = ""
        usage = {}
        finish_reason = None
        if response:
            choice = response.get("choices", [{}])[0]
            message = choice.get("message", {})
            content = message.get("content") or ""
            reasoning_content = message.get("reasoning_content") or ""
            usage = response.get("usage") or {}
            finish_reason = choice.get("finish_reason")

        validation = validate_text(content) if content else {
            "valid": False,
            "errors": ["No final content returned."],
            "warnings": [],
            "object_count": 0,
        }
        completion_tokens = usage.get("completion_tokens", 0)
        output_tokens_per_second = (
            completion_tokens / duration if completion_tokens and duration else None
        )
        run_record = {
            "config": config,
            "duration_seconds": duration,
            "output_tokens_per_second": output_tokens_per_second,
            "finish_reason": finish_reason,
            "usage": usage,
            "error": error,
            "validation": validation,
            "content": content,
            "reasoning_content": reasoning_content,
        }
        run_path = args.output_dir / f"{config['id']}.json"
        run_path.write_text(
            json.dumps(run_record, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )
        if content:
            (args.output_dir / f"{config['id']}.xml").write_text(
                content,
                encoding="utf-8",
            )
        summaries.append(
            {
                "id": config["id"],
                "duration_seconds": round(duration, 3),
                "output_tokens_per_second": (
                    round(output_tokens_per_second, 3)
                    if output_tokens_per_second is not None
                    else None
                ),
                "finish_reason": finish_reason,
                "usage": usage,
                "valid": validation["valid"],
                "error_count": len(validation["errors"]),
                "request_error": error,
            }
        )
        print(json.dumps(summaries[-1], ensure_ascii=False))

    summary_path = args.output_dir / "summary.json"
    existing_summaries = []
    if summary_path.exists():
        try:
            existing_summaries = json.loads(summary_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            pass
    by_id = {
        item["id"]: item
        for item in existing_summaries
        if isinstance(item, dict) and "id" in item
    }
    by_id.update({item["id"]: item for item in summaries})
    ordered_summaries = [
        by_id[item["id"]] for item in MATRIX if item["id"] in by_id
    ]
    summary_path.write_text(
        json.dumps(ordered_summaries, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
