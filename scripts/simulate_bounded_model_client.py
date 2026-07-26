#!/usr/bin/env python3
"""Run one bounded, stateless KSML request against an OpenAI-compatible API."""

from __future__ import annotations

import argparse
import json
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from pathlib import Path


SYSTEM_PROMPT = """/no_think
You generate TeaQL KSML. Return one complete XML document only.
Do not use tools, search, Markdown fences, commentary, or explanations.
Follow the supplied grammar and value whitelist exactly."""

USER_PROMPT = """Create a small school-service KSML model with exactly three entities:

1. platform
   - name="string()"
   - platform_code="string()"
   - created_at="createTime()"
2. merchant
   - name="string()"
   - merchant_code="string()"
   - contact_email="string()"
   - platform="platform()"
3. school
   - name="string()"
   - school_code="string()"
   - address="string()"
   - merchant="merchant()"

Use meaningful field names exactly as listed. Do not invent attr1/attr2.

Required grammar example:
<root name="example-service" data_service="sqlite">
  <account _name="Account" _module="Accounts" _module_key="accounts"
           name="string()" created_at="createTime()"/>
</root>

Rules:
- The root element must be <root name="school-service" data_service="sqlite">.
- Entity names are element tags; fields and relationships are XML attributes.
- Every entity must contain _name, _module, and _module_key.
- Do not use <object>, <field>, nested field elements, id, or integer().
- Allowed scalar values: string(), createTime(), updateTime().
- Allowed relationship values: platform(), merchant().
- Output XML only."""

EXPECTED_FIELDS = {
    "platform": {
        "name": "string()",
        "platform_code": "string()",
        "created_at": "createTime()",
    },
    "merchant": {
        "name": "string()",
        "merchant_code": "string()",
        "contact_email": "string()",
        "platform": "platform()",
    },
    "school": {
        "name": "string()",
        "school_code": "string()",
        "address": "string()",
        "merchant": "merchant()",
    },
}


def post_json(url: str, payload: dict, timeout: int) -> dict:
    request = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return json.loads(response.read().decode("utf-8"))


def extract_xml(content: str) -> str:
    text = content.strip()
    if "```xml" in text:
        text = text.split("```xml", 1)[1].split("```", 1)[0].strip()
    elif "```" in text:
        text = text.split("```", 1)[1].split("```", 1)[0].strip()
    start = text.find("<?xml")
    if start < 0:
        start = text.find("<root")
    if start >= 0:
        text = text[start:]
    end = text.rfind("</root>")
    if end >= 0:
        text = text[: end + len("</root>")]
    return text


def validate_xml(xml_text: str) -> dict:
    errors: list[str] = []
    warnings: list[str] = []
    try:
        root = ET.fromstring(xml_text)
    except ET.ParseError as exc:
        return {"valid": False, "errors": [f"XML parse error: {exc}"], "warnings": []}

    if root.tag != "root":
        errors.append(f"Expected root tag 'root', got {root.tag!r}.")
    if root.attrib.get("name") != "school-service":
        errors.append("Root name must be 'school-service'.")
    if root.attrib.get("data_service") != "sqlite":
        errors.append("Root data_service must be 'sqlite'.")

    children = list(root)
    names = [child.tag for child in children]
    if names != list(EXPECTED_FIELDS):
        errors.append(
            f"Expected entity order {list(EXPECTED_FIELDS)!r}, got {names!r}."
        )

    for entity_name, expected in EXPECTED_FIELDS.items():
        matches = [child for child in children if child.tag == entity_name]
        if len(matches) != 1:
            errors.append(
                f"Expected exactly one {entity_name!r} entity, got {len(matches)}."
            )
            continue
        entity = matches[0]
        if list(entity):
            errors.append(f"{entity_name!r} must not contain nested field elements.")
        for metadata in ("_name", "_module", "_module_key"):
            if not entity.attrib.get(metadata):
                errors.append(f"{entity_name!r} is missing {metadata!r}.")
        for field_name, value in expected.items():
            actual = entity.attrib.get(field_name)
            if actual != value:
                errors.append(
                    f"{entity_name}.{field_name} expected {value!r}, got {actual!r}."
                )
        allowed = {"_name", "_module", "_module_key", *expected}
        extras = sorted(set(entity.attrib) - allowed)
        if extras:
            warnings.append(f"{entity_name!r} added attributes: {extras!r}.")

    return {"valid": not errors, "errors": errors, "warnings": warnings}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--model", default="nemotron-3-super")
    parser.add_argument("--max-tokens", type=int, default=2048)
    parser.add_argument("--timeout", type=int, default=300)
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("artifacts/python-client-simulation-20260727"),
    )
    args = parser.parse_args()
    args.output_dir.mkdir(parents=True, exist_ok=True)

    messages = [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": USER_PROMPT},
    ]
    payload = {
        "model": args.model,
        "messages": messages,
        "temperature": 0.0,
        "top_p": 1.0,
        "max_tokens": args.max_tokens,
        "chat_template_kwargs": {"enable_thinking": False},
    }
    request_record = {
        "endpoint_path": "/v1/chat/completions",
        "payload": payload,
        "stateless": True,
        "tool_count": 0,
    }
    (args.output_dir / "request.json").write_text(
        json.dumps(request_record, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    started = time.monotonic()
    try:
        response = post_json(
            args.base_url.rstrip("/") + "/v1/chat/completions",
            payload,
            args.timeout,
        )
        request_error = None
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as exc:
        response = {}
        request_error = f"{type(exc).__name__}: {exc}"
    duration = time.monotonic() - started

    (args.output_dir / "response.json").write_text(
        json.dumps(response, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    choice = (response.get("choices") or [{}])[0]
    message = choice.get("message") or {}
    content = message.get("content") or ""
    xml_text = extract_xml(content) if content else ""
    if xml_text:
        (args.output_dir / "model.xml").write_text(xml_text + "\n", encoding="utf-8")
    validation = (
        validate_xml(xml_text)
        if xml_text
        else {"valid": False, "errors": ["No XML content returned."], "warnings": []}
    )
    usage = response.get("usage") or {}
    completion_tokens = usage.get("completion_tokens") or 0
    summary = {
        "duration_seconds": round(duration, 3),
        "request_error": request_error,
        "finish_reason": choice.get("finish_reason"),
        "usage": usage,
        "output_tokens_per_second": (
            round(completion_tokens / duration, 3)
            if completion_tokens and duration
            else None
        ),
        "content_characters": len(content),
        "reasoning_characters": len(message.get("reasoning_content") or ""),
        "validation_scope": "local exact-structure and field-value checks",
        "validation": validation,
    }
    (args.output_dir / "summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    print(json.dumps(summary, ensure_ascii=False, indent=2))
    return 0 if validation["valid"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
