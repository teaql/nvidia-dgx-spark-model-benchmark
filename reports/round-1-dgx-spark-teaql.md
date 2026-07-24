# Round 1: DGX Spark and TeaQL Agent Kit Feasibility

> Collected: 2026-07-24 05:47:48 UTC
> Host: `spark-007`
> Stage: smoke test and feasibility
> Raw data: [system-snapshot.json](../artifacts/round-1/system-snapshot.json)

## Executive result

Round 1 confirmed that VLLM on DGX Spark can provide stable OpenAI-compatible
Chat Completions and Responses APIs. With thinking disabled, the `omni` model
returned the first streaming byte in 0.187 seconds and completed a minimal
request in 1.023 seconds. A real TeaQL dependency-diagnosis request completed
in 5.84 seconds.

Task quality was not yet sufficient for autonomous TeaQL engineering:

- With default thinking, both `omni` and `nemotron` emitted reasoning into the
  response body, exhausted a 1,400-token output limit, and produced no KSML.
- With thinking disabled, both models responded faster but generated malformed
  XML or violated TeaQL modeling rules.
- On a Rust/Cargo dependency-drift task, `omni` identified the broad problem
  but misunderstood Cargo caret semantics and proposed an ineffective repair.
- After receiving exact validation feedback, it still failed to repair the
  answer and invented a workspace configuration absent from the repository.
- A standard Codex CLI prompt plus tool schemas exceeded the service's
  8,192-token context before model inference began.

The Round 1 conclusion was therefore: DGX Spark inference was operational with
acceptable latency, but the tested models and configuration required bounded
tasks, deterministic validation, and supervised repair.

## Test scope

The round covered:

1. A small TeaQL modeling task for structure and rule-following.
2. A Rust/Cargo diagnosis task after modeling and generation.
3. Model-service, system-resource, toolchain, and stability measurements.

No model suggestion was applied to a working project, and no KSML or generated
source was modified during this round.

## DGX Spark environment

| Item | Value |
|---|---|
| Hardware | NVIDIA DGX Spark |
| GPU | NVIDIA GB10 |
| Architecture | ARM64 / AArch64 |
| CPU | 20 cores: 10 Cortex-X925 and 10 Cortex-A725 |
| NUMA | One node |
| Memory | 121.7 GiB unified memory |
| Swap | 16.0 GiB |
| System disk | 3.67 TiB NVMe ext4 |
| OS | Ubuntu 24.04.4 LTS |
| Kernel | Linux 6.17.0-1026-nvidia |
| NVIDIA driver | 580.159.03 |
| CUDA | 13.0, build 13.0.88 |
| Docker | 29.2.1 |
| Docker Compose | 5.0.2 |
| Python | 3.12.3 |
| Rust | 1.97.1 |
| Cargo | 1.97.1 |
| Git | 2.43.0 |

The host had been up for 7 days and 15 hours. Load average was
0.67 / 0.51 / 0.38.

### Resource snapshot

| Metric | Value |
|---|---:|
| System memory used | 54 GiB |
| System memory available | 66 GiB |
| Swap used | 352 MiB |
| Root filesystem | 129 GiB / 3.7 TiB, 4% |
| GPU temperature | 44°C |
| GPU utilization | 0% at idle snapshot |
| GPU power | 11.58 W |
| VLLM EngineCore unified-memory use | 44,732 MiB |
| Docker cgroup memory display | 4.502 GiB |

DGX Spark uses unified memory, so Docker cgroup values do not represent the
complete model allocation. Capacity decisions must combine system memory,
`nvidia-smi` process information, and VLLM load logs.

## Model-service configuration

Only `omni-server` remained active at the end of Round 1.

| Item | Value |
|---|---|
| Served model | `omni` |
| Architecture | `NemotronH_Nano_Omni_Reasoning_V3` |
| Model size on disk | 21 GiB |
| Quantization | ModelOpt / mixed NVFP4 |
| Compute dtype | bfloat16 |
| Image | `vllm/vllm-openai:nightly` |
| VLLM | `0.23.1rc1.dev1181+g6a9f24aa8` |
| APIs | Chat Completions and Responses |
| Context limit | 8,192 tokens |
| Maximum sequences | 4 |
| GPU-memory utilization | 0.4 |
| Restart policy | `unless-stopped` |

### Cold start

| Metric | Value |
|---|---:|
| Container start | 05:37:18 UTC |
| API ready | Approximately 05:39:33 UTC |
| Start-to-ready | Approximately 135 seconds |
| Weight loading | 98.70 seconds |
| VLLM model memory | 21.59 GiB |
| Available KV cache | 20.99 GiB |
| KV token capacity | 3,529,728 |

Schedulers must poll `/health` or `/v1/models`; container `running` status is
not an adequate readiness signal.

## Modeling smoke test

The fixed task requested a small school-registration KSML model containing
`platform`, `school_type`, `school`, and `registration`.

| Model | Mode | Elapsed | Prompt | Completion | Result |
|---|---|---:|---:|---:|---|
| omni | Default thinking | 22.82 s | 195 | 1,400 | Truncated; no XML |
| nemotron | Default thinking | 98.82 s | 179 | 1,400 | Truncated; no XML |
| omni | `/no_think` | 4.53 s | 212 | 235 | Invalid XML/KSML |
| nemotron | `/no_think` | 12.11 s | 198 | 168 | Invalid XML/KSML |

Observed failures included unquoted XML attributes, invalid business-object
metadata, nested object definitions, malformed constant `_value` entries,
invalid relationships, and invalid system-time fields.

Disabling thinking improved latency but did not solve rule adherence. The next
architecture therefore supplied a validated model before asking the DGX model
to write application logic.

## Standard Codex CLI integration

Codex CLI 0.144.6 was installed and VLLM implemented `/v1/responses`, but the
first request failed before inference:

```text
maximum context length is 8192 tokens
prompt contains at least 8193 input tokens
```

The full system prompt and tool schema already exceeded 8K. Standard agent use
required at least 16K, preferably 32K, followed by another KV-cache and memory
assessment.

## Rust/Cargo dependency diagnosis

The test used a generated FIFA World Cup 2026 Rust example. Its source expected
TeaQL 4.0.3 APIs, while Cargo resolved `4.1.1`, producing 69 errors around
removed repository types and registry APIs.

The real cause was Cargo's caret behavior: `"4.0.3"` means `^4.0.3`, allowing
versions from 4.0.3 up to, but not including, 5.0.0. Without a controlled lock
file, 4.1.1 was selected and broke generated-code compatibility.

The first `omni` diagnosis took 5.84 seconds for 214 prompt and 313 completion
tokens. It recognized version mismatch but incorrectly described caret
semantics and recommended regenerating a lock file, which would still resolve
4.1.1. It also named a nonexistent package.

After exact feedback, the repair took 2.86 seconds for 293 prompt and 89
completion tokens. It still failed to provide the required command and invented
a `[workspace]` configuration not present in the project.

## Streaming readiness request

| Metric | Value |
|---|---:|
| HTTP status | 200 |
| First byte | 0.187 s |
| Total time | 1.023 s |
| Prompt tokens | 39 |
| Completion tokens | 3 |
| Total tokens | 42 |

This request only returned `OK`; it is a readiness measurement, not an
engineering-performance result.

## Operational events

1. Running `omni-server` and `nemotron-server` simultaneously used about
   95 GiB and left roughly 26 GiB available.
2. Both containers later received explicit stop events and exited with code
   zero; neither was OOM-killed.
3. Restoring only `omni-server` left approximately 66 GiB available.
4. A user-level HTTPS proxy was configured and verified, then removed by an
   external rewrite of shell configuration files. It was restored and returned
   HTTP 200. Future automation should avoid assuming dotfiles remain unchanged.

## Limitations

- This was a smoke test with a small sample, not a statistical benchmark.
- Non-streaming elapsed time is not time-to-first-token.
- The streaming measurement used one minimal request.
- The VLLM nightly image was mutable and should be pinned by digest.
- Reasoning and tool-call parsers were not configured.
- The 8K context could not host a standard Codex tool environment.
- Unified memory makes discrete-GPU memory comparisons misleading.
- No model patch was applied, so edit/compile/repair success was not measured.

## Round 1 scorecard

| Dimension | Result |
|---|---|
| API availability | PASS |
| Single-model stability | PASS |
| Minimal-request latency | PASS |
| TeaQL modeling | FAIL |
| Cargo diagnosis accuracy | FAIL |
| Feedback-driven self-repair | FAIL |
| Standard Codex integration | BLOCKED by 8K context |
| Unsupervised code modification | Not recommended |

The recommended next architecture was to use deterministic model validation and
generation, give the DGX model bounded application tasks, validate every patch
with XML/compiler/tests, and raise context to 32K before expanding the task set.

## Security

No artifact includes SSH passwords, API keys, proxy credentials, public host
addresses, or unrelated sensitive environment variables. Remote endpoints are
represented as `<DGX_HOST>`.
