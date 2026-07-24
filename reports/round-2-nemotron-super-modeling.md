# Round 2: Nemotron 3 Super 120B TeaQL Modeling

> Date: 2026-07-24
> Stage: 120B NVFP4 startup and TeaQL KSML parameter exploration
> Raw outputs: [modeling-runs](../artifacts/round-2/modeling-runs/)
> System snapshot: [system-snapshot.json](../artifacts/round-2/system-snapshot.json)
> Fixed task: [school-platform-task.txt](../benchmarks/round-2/school-platform-task.txt)

## Executive result

`NVIDIA-Nemotron-3-Super-120B-A12B-NVFP4` started successfully on one DGX
Spark using a VLLM nightly build. It completed four first-attempt modeling
requests with a 4.3K-token TeaQL rules prompt and one feedback repair. Warm
generation remained between 15.45 and 16.36 tokens per second. All five
requests returned HTTP 200 without restarting the container.

None of the four first attempts passed the fixed KSML validator:

- The best configuration disabled thinking and used `temperature=1.0`,
  `top_p=0.95`. Its XML parsed and all 13 objects, relationships, and constant
  values were correct. The only error was a prohibited `employee_id` field.
- Greedy and low-temperature configurations emitted unquoted numeric XML
  attributes. After diagnostic syntax repair, the greedy output also lacked
  root ownership on four constants.
- Thinking with a requested 1,024-token reasoning budget performed worst:
  328 seconds and 5,370 completion tokens, no separate `reasoning_content`,
  and several unquoted attributes in the final XML.

When the best output and its one exact validator error were returned to the
model, it changed `employee_id` to `staff_number`; the complete XML passed all
fixed checks in one repair. The final conclusion was that the 120B model had
strong structural understanding and could repair precise feedback, but first
attempts still required deterministic validation before code generation.

## Model and checkpoint

| Item | Value |
|---|---|
| Model | NVIDIA Nemotron 3 Super 120B A12B |
| Total parameters | Approximately 120.6B |
| Active parameters per token | Approximately 12.7B |
| Architecture | LatentMoE, Mamba2 + MoE + Attention |
| Layers | 88 |
| Experts | 512, top-22 |
| Checkpoint | NVIDIA ModelOpt mixed / NVFP4 |
| Local shards | 17 / 17 |
| Shard bytes | 80,317,948,856 |
| VLLM-reported weights | 74.80 GiB |
| Model-config context | 262,144 |
| Served context | 32,768 |

Startup selected `FlashInferCutlassNvFp4LinearKernel` and the
`FLASHINFER_CUTLASS` NVFP4 MoE backend. Compute dtype was bfloat16 and KV cache
used FP8.

## Service configuration

| Parameter | Value |
|---|---|
| Image | `vllm/vllm-openai:nightly` |
| VLLM | `0.23.1rc1.dev1181+g6a9f24aa8` |
| Served model | `nemotron-3-super` |
| `max-model-len` | 32,768 |
| `max-num-seqs` | 1 |
| `gpu-memory-utilization` | 0.9 |
| KV cache dtype | FP8 |
| Mamba SSM cache dtype | float16 |
| Chunked prefill | Enabled |
| Async scheduling | Enabled |
| Reasoning parser | `super_v3` plugin |
| Tool parser | `qwen3_coder` |
| Restart policy | `unless-stopped` |

The service had no API key and its Docker port was published. A persistent
deployment should restrict the listener or firewall sources and add
authentication.

## Cold start and resources

| Metric | Value |
|---|---:|
| Container start to API ready | Approximately 686 seconds |
| Weight loading | 388.69 seconds |
| VLLM model memory | 69.62 GiB |
| Engine initialization and warm-up | 243.71 seconds |
| Torch compilation | 21.07 seconds |
| CUDA graph memory | 1.54 GiB |
| KV cache | 39.02 GiB |
| Initial FlashInfer tuning | 170 configurations |
| Idle temperature | 53°C |
| Idle power | 12.92 W |

Only 1.81 GiB of system memory remained available and 6.82 GiB of swap was in
use. `docker stats` showed only 3.137 GiB, demonstrating again that cgroup
memory is incomplete on GB10 unified memory. This 0.9 utilization profile was
not suitable for simultaneous heavy Cargo builds or another model server.

## Fixed modeling task

The task requested a platform-managed school enrollment and course-selection
model with exactly 13 direct children:

- Nine business objects: Platform, School, Campus, Prospective Student,
  Admission Application, Guardian, Teacher, Course, and Enrollment.
- Four constants: School Type, Application Status, Course Category, and
  Enrollment Status.

All configurations used the same task, complete `ksml-rules.md`, and system
instruction. [`validate_ksml.py`](../scripts/validate_ksml.py) performed strict
XML and task-contract checks.

## Parameter matrix

| Configuration | Thinking | temp / top_p | Elapsed | Prompt | Completion | tok/s | Result |
|---|---|---:|---:|---:|---:|---:|---|
| no-think-greedy | Off | 0 / 1.0 | 78.58 s | 4,312 | 1,214 | 15.45 | FAIL |
| no-think-official-sampling | Off | 1.0 / 0.95 | 68.48 s | 4,312 | 1,090 | 15.92 | FAIL, closest |
| no-think-low-temperature | Off | 0.2 / 0.9 | 77.99 s | 4,312 | 1,249 | 16.02 | FAIL |
| think-budget-1024 | On | 1.0 / 0.95 | 328.15 s | 4,309 | 5,370 | 16.36 | FAIL |

For the three no-thinking runs, mean end-to-end time was 75.02 seconds and mean
throughput was approximately 15.79 tokens per second.

### Greedy

The main syntax error was an unquoted `credits=3`. After correcting only that
value in memory for diagnosis, all 13 objects existed but four constant objects
lacked `platform="platform()"`.

### Official sampling

This was the strongest first attempt. XML, objects, relationships, time fields,
constant codes and IDs, and platform ownership all passed. Its sole error was:

```text
teacher.employee_id: relationship uses _id suffix.
```

The model reproduced the complex skeleton but violated a local rule while
adding a plausible business field.

### Low temperature

This output combined `credits=3`, `employee_id`, and missing platform ownership
on four constants. Lower temperature did not improve determinism.

### Thinking budget

The request ended normally with `finish_reason=stop`, yet used 5,370 completion
tokens and returned no separate reasoning field. The tested VLLM nightly,
chat template, and `super_v3` parser did not expose or enforce the requested
reasoning budget as expected. Quality and latency were both worse.

### Exact-feedback repair

The best output, complete original XML, and its single error were returned with
thinking disabled and official sampling:

| Metric | Value |
|---|---:|
| Elapsed | 68.92 seconds |
| Prompt tokens | 5,464 |
| Completion tokens | 1,090 |
| Throughput | 15.82 tokens/s |
| Errors before | 1 |
| Errors after | 0 |
| Result | PASS |

The model preserved every correct structure and changed only
`teacher.employee_id` to `teacher.staff_number`. The final result had 13 direct
objects, nine business objects, and four constants.

## Risks

1. FP8 KV logs reported an uncalibrated q/prob scale of 1.0; compare quality
   with BF16 or FP16 KV.
2. VLLM used a default Mamba SSU configuration for GB10 and warned that
   performance may be suboptimal.
3. Two legacy environment variables were ignored by the nightly build; remove
   them from startup scripts.
4. Pin the mutable nightly image by digest.
5. A 0.9 memory target forced substantial swap and reduced build headroom.
6. The tested reasoning-budget behavior should not be enabled in agent runs.

## Recommended next steps

Start modeling with:

```text
enable_thinking=false
temperature=1.0
top_p=0.95
```

Use a bounded generate → validate → exact-feedback → revalidate loop with at
most two repairs. Explicitly prohibit `_id` attribute names and require quoted
XML attributes.

Before non-modeling tasks, restart with 4–8 GiB explicit KV cache so system
memory remains available for generation, Cargo compilation, and tests. Then
compare BF16/FP16 and FP8 KV quality, repeat repaired-generation trials, and
measure pass@1, pass@2, TTFT, elapsed time, and peak memory across fixed tasks.

## Artifacts

- [Raw parameter runs and repair run](../artifacts/round-2/modeling-runs/)
- [Machine-readable matrix summary](../artifacts/round-2/modeling-runs/summary.json)
- [System snapshot](../artifacts/round-2/system-snapshot.json)
- [`run_modeling_matrix.py`](../scripts/run_modeling_matrix.py)
- [`validate_ksml.py`](../scripts/validate_ksml.py)

Raw response files are intentionally preserved in their original form. No
artifact contains SSH passwords, proxy credentials, or a public service
address.
