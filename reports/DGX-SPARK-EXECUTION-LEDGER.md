# DGX Spark Execution Ledger

> Evidence window: 2026-07-24 to 2026-07-25 (Asia/Shanghai)
>
> Prepared from committed reports, machine-readable summaries, generated
> artifacts, evaluation logs, scripts, and Git history. This ledger does not
> treat a final repaired artifact as a model pass at first attempt.

## Executive finding

The evidence contains three materially different execution paths:

1. **Workstation MiMoCode 0.1.7 path:** transport, Chat Completions, structured
   tool calls, local file reads, and local shell execution were validated.
   Autonomous end-to-end execution was not reliable. Round 3 reached a repaired
   pass but failed pass@1 and the headless client did not exit. Round 4
   exhausted the model context, and its recovery session never performed the
   requested transformation; deterministic local scripts produced the final
   model.
2. **Python orchestration path:** scripts called the DGX-hosted
   OpenAI-compatible endpoint directly, wrote model responses locally, and ran
   TeaQL and Cargo locally. This path produced the strongest repeatable record:
   modular Nemotron generation succeeded through 140 objects and failed at 160
   objects because of 600-second request timeouts and a circular dependency.
3. **DGX-local MiMoCode 0.1.9 path:** a later launcher ran MiMoCode on
   `spark-007` against the local VLLM port. Its main session invoked
   checkpoint-writer subagents that produced two retry/message storms. The
   larger subagent session accumulated 6,864 assistant messages. Its confirmed
   error requested at least 65,537 tokens against the 65,536-token model limit,
   followed by rapid retries without effective backoff.

The defensible conclusion is therefore not “MiMoCode never worked.” Its
integration smoke test worked, while its autonomous task execution was
incomplete or inefficient. Most successful large-scale results came from
Python API orchestration plus deterministic validation and repair.

## Evidence classification

| Classification | Meaning |
|---|---|
| Confirmed DGX inference | A report or machine-readable record identifies the DGX model service and records a model/API result. |
| Repaired final pass | The checked-in final result passed, but model pass@1 failed or deterministic repair was required. |
| Local-only or unconfirmed | Local generation, validation, compilation, or simulation is recorded, but successful DGX inference is not established. |
| Failed | The intended model or agent execution did not reach the requested acceptance gate. |

## Chronological execution record

| Date | Run | Execution path | Recorded outcome | Classification | Primary evidence |
|---|---|---|---|---|---|
| 2026-07-24 | Round 1 service and modeling smoke tests | Direct API and Codex CLI probe | Chat Completions, Responses, and streaming health worked. Four modeling attempts failed. Standard Codex could not fit its prompt and tools into the 8K context. | Confirmed DGX inference; task failed | `artifacts/round-1/system-snapshot.json`, `reports/round-1-dgx-spark-teaql.md` |
| 2026-07-24 | Round 2 parameter matrix | `scripts/run_modeling_matrix.py` calling Nemotron | Four initial configurations were invalid. Exact validator feedback repaired the official-sampling result from one error to zero. | Repaired final pass | `artifacts/round-2/modeling-runs/summary.json`, `reports/round-2-nemotron-super-modeling.md` |
| 2026-07-24 | MiMoCode integration smoke | MiMoCode 0.1.7 with `scripts/mimo-dgx` | Local `read` and `bash` tool calls passed against the TeaQL workspace. The server was ready at 64K context. This was a read-only integration check, not a coding-task completion. | Confirmed DGX inference; integration pass | `artifacts/round-2/local-agent-smoke.json`, `reports/local-mimocode-dgx-agent.md` |
| 2026-07-24 | Round 3 school Q/E task | MiMoCode agent using DGX Nemotron, local TeaQL/Cargo/SQLite | Final evaluate, check, test, and run passed after compiler- and runtime-guided repair. Pass@1 failed with eight compiler errors; a compiled string value persisted as numeric zero; MiMo headless remained alive after completion. | Repaired final pass | `artifacts/round-3/summary.json`, `reports/round-3-nemotron-agent-rust-teaql.md` |
| 2026-07-24 | Round 4 223-object moving-company task | MiMoCode modeling sessions followed by deterministic migration and focused local implementation | Initial session exceeded 65,536 tokens without changing the model. Recovery repeatedly listed directories and never transformed the model. Deterministic migration and small repairs produced zero evaluation errors and 10 passing tests. | MiMoCode model execution failed; repaired final pass | `artifacts/round-4/summary.json`, `reports/round-4-nemotron-moving-company.md` |
| 2026-07-24/25 | Round 5 193-object prepared model | `prepare_round5_model.py`, validators, TeaQL, and Cargo | Prepared model evaluated with zero errors; generated Rust compiled and Q/E runtime checks passed. The checked-in evidence establishes the local deterministic pipeline, not a successful DGX generation pass. | Local-only or unconfirmed DGX inference | `artifacts/round-5/evaluate.log`, `reports/round-5-run-report.md` |
| 2026-07-24/25 | Round 6 operations bounded context | Local modeling, TeaQL, and Cargo | A 27-object model evaluated with zero errors and completed local Q/E checks. | Local-only or unconfirmed DGX inference | `artifacts/round-6/evaluate.log`, `reports/round-6-run-report.md` |
| 2026-07-24/25 | Round 7 HR/payroll bounded context | Intended Qwen request, then local simulation | The remote model path returned 404. The report explicitly says the schema was simulated locally; local evaluation and runtime passed after two reserved-keyword fixes. | DGX request failed; local final pass | `reports/round-7-run-report.md`, `artifacts/round-7/evaluate.log` |
| 2026-07-24/25 | Round 8 Qwen 180-object monolith, 128K | `scripts/run_round8_model.py` calling a DGX-hosted Qwen service | About 15K output tokens took about 12 minutes. Raw output had 732 structural errors. Heavy AST repair reached zero XML errors, but Rust relation semantics were inconsistent and testing was aborted. A separate Claude wrapper log also records model selection/access failure. | Confirmed DGX inference; end-to-end failed | `reports/round-8-run-report.md`, `artifacts/round-8/evaluate.log`, `artifacts/round-8/claude-dgx.log` |
| 2026-07-24/25 | Round 9 Qwen 180-object monolith, 64K | `scripts/run_round9_model.py` | About 15K output tokens took about 15 minutes. Initial evaluation had 948 errors and the graph contained a depth-20 circular dependency. | Confirmed DGX inference; failed | `reports/round-9-run-report.md`, `artifacts/round-9/evaluate.log` |
| 2026-07-24/25 | Original Rounds 10 and 11 monolith tests | `scripts/run_round10.py` and `scripts/run_round11.py` | The 90-object output required aggressive structural transformation. The 60-object output degenerated into an infinite repetition loop. | Confirmed DGX inference; model generation failed or required heavy salvage | `reports/round-10-11-run-report.md`, round 10/11 evaluation logs |
| 2026-07-24/25 | Modular reruns of Rounds 10 and 11 | `scripts/dgx_round10_agent.py`, `scripts/dgx_round11_agent.py` | Splitting generation across files produced zero-error TeaQL results at 90 and 60 objects; Round 11 needed one reserved-keyword fix. | Confirmed DGX inference; repaired final pass | `reports/modular-dgx-nemotron-benchmark.md`, round 10/11 modular artifacts |
| 2026-07-24/25 | Round 12 Qwen 30-object service | `scripts/run_round12.py` | Focused generation completed in 157 seconds but had 202 native syntax errors. AST normalization and a privacy fix produced zero evaluation errors. | Confirmed DGX inference; repaired final pass | `reports/round-12-run-report.md`, `artifacts/round-12/evaluate.log` |
| 2026-07-24/25 | Round 13 Nemotron 30-object service | `scripts/run_round13.py` | Focused generation completed in about 173 seconds but had 185 native syntax errors. AST normalization and a privacy fix produced zero evaluation errors. | Confirmed DGX inference; repaired final pass | `reports/round-13-run-report.md`, `artifacts/round-13/evaluate.log` |
| 2026-07-25 | Round 14 Nemotron 180-object modular model | `scripts/dgx_round14_agent.py`, 14 concurrent requests | Fourteen generated modules were assembled with TeaQL includes. Removing model-prompted implicit `id` and `name` fields enabled zero-error evaluation and Rust compilation. | Confirmed DGX inference; repaired final pass | `reports/modular-dgx-nemotron-benchmark.md`, `artifacts/round-14/` |
| 2026-07-25 | Phase 2 Rounds 15–26, 10 to 140 objects | `scripts/run_round.py` and `scripts/run_all.sh`, direct Nemotron API with local repair/compile loop | All twelve checked-in rounds have XML entrypoints, generated library/application archives, extracted Rust application workspaces, and success reports. The orchestrator limited generation concurrency to two and retried each module and round. | Confirmed DGX inference; scripted final passes | `reports/Round-15-Report.md` through `Round-26-Report.md`, `artifacts/round-15/` through `round-26/` |
| 2026-07-25 | Phase 2 Round 27, 160 objects | Same direct API orchestrator with feedback loop | Failed after cascading 600-second timeouts and a depth-20 circular dependency. The directory contains XML but no generated Rust archives or Cargo workspace, matching failure before compilation. | Confirmed DGX inference; failed | `reports/BENCHMARK-CONCLUSION.md`, `reports/DGX-GPU-TELEMETRY.md`, `artifacts/round-27/` |
| 2026-07-25 | DGX-local MiMoCode 0.1.9 execution | `/home/dgx007/start-mimo-local.sh`, local client calling local port 30704 | The main session ran from 03:18:59 to 05:47:46 UTC. Two checkpoint-writer subagents generated 3,420 and 6,864 assistant messages. In the latter, 847 processor errors explicitly report 57,345+ input tokens plus 8,192 requested output tokens, one token beyond the 65,536 context. The client retried rapidly and did not recover. | Confirmed DGX-local client execution; failed | `artifacts/dgx-remote-verification-20260726.json`, remote MiMoCode log/database aggregates |
| 2026-07-27 | Bounded three-object modeling test | DGX-local MiMoCode 0.1.9 build agent with built-in skills disabled and a 10-minute watchdog | The original local launcher failed because `local-vllm/nemotron-3-super` was not registered, while MiMoCode incorrectly exited 0. Retrying with `dgx/nemotron-3-super` avoided broad discovery and context errors, but still invoked `skill_search`, generated invalid root/field syntax, and did not exit after writing. TeaQL reported 15 errors, 2 warnings, and 9 solids. | Confirmed DGX-local inference; failed | `artifacts/simple-model-test-20260727/summary.json`, `artifacts/simple-model-test-20260727/raw-model.xml` |
| 2026-07-27 | Bounded modeling A/B refinement | Same three-object task and scope; first add an unrelated minimal grammar example, then add a verified value-form whitelist | The grammar example reduced TeaQL errors from 15 to 1. Restricting values to verified forms reduced the result to 0 errors, 4 warnings, and 10 solids. All attempts avoided broad repository discovery and context overflow. Every attempt still invoked `skill_search` and required controlled termination after writing. | Confirmed DGX-local inference; structural pass with quality caveats | `artifacts/simple-model-test-20260727/ab-summary.json`, syntax-example and scalar-whitelist XML artifacts |
| 2026-07-27 | Bounded direct Python client simulation | A stateless Python `urllib` client called the DGX-hosted OpenAI-compatible endpoint directly with no tools, no history, thinking disabled, and a 2,048-token output ceiling | The request used 335 prompt tokens and 138 completion tokens, returned `finish_reason=stop` in 10.155 seconds, and produced the exact three requested entities and meaningful fields. Local exact-structure checks reported zero errors and zero warnings. The official TeaQL evaluator then reported 0 errors, 10 warnings, 0 suggestions, and 10 solids; both references resolved and exactly one domain root was found. | Confirmed DGX inference; official TeaQL structural pass | `artifacts/python-client-simulation-20260727/summary.json`, `teaql-evaluate.md`, `model.xml`, `request.json`, and `response.json` |
| 2026-07-27 | Bounded generate-validate-repair pipeline | A stateless Python runner combined an explicit task package, context admission control, local XML and exact-task checks, official TeaQL evaluation, a fresh-session repair path, and conditional finalization | Fresh generation used 440 prompt and 138 completion tokens and passed directly. A separate repair test rejected the earlier invalid MiMoCode XML locally, used one 898-prompt-token repair request, and passed. Both paths finished with 0 local errors and official TeaQL results of 0 errors, 10 warnings, 0 suggestions, and 10 solids; their final models were byte-identical. | Confirmed DGX inference; fresh and repair paths passed | `reports/bounded-model-pipeline.md`, `artifacts/bounded-model-pipeline-20260727/`, `artifacts/bounded-model-repair-test-20260727/` |
| 2026-07-27 | Moving-company 30-object bounded pipeline | The supplied broad platform scope was reduced to an explicit 30-object task package with exact object, field, relationship, module, versioning, soft-delete, and audit requirements | A preliminary candidate passed basic structure but had three inappropriate module assignments. The strengthened acceptance spec rejected it with seven errors. One fresh repair request used 3,981 prompt and 2,027 completion tokens, completed in 127.399 seconds, and produced 30 accepted objects across eight modules. After explicit export authorization, the official TeaQL evaluator reported 0 errors, 141 warnings, 6 suggestions, and 67 solids; all 34 relationships resolved. | Confirmed DGX inference; local exact-acceptance and official TeaQL structural pass | `reports/moving-company-30-pipeline.md`, `artifacts/moving-company-30-pipeline-20260727/` |

## Remote DGX verification

A read-only SSH inspection at 2026-07-26 17:02 UTC confirmed:

- host `spark-007`, architecture `aarch64`, NVIDIA GB10, driver 580.159.03;
- active container `nemotron-super-server-64k` on published port 30704;
- VLLM nightly `0.23.1rc1.dev1181+g6a9f24aa8`, 65,536-token context,
  one sequence, FP8 KV cache, chunked prefill, asynchronous scheduling,
  `qwen3_coder` tool parser, and `super_v3` reasoning parser;
- container start at 2026-07-24 15:07:53 UTC and application readiness at
  15:19:30 UTC, including 392.74 seconds to load model weights;
- 500 HTTP 200 and 10,241 HTTP 400 Chat Completions responses in the active
  container log; successful requests span 2026-07-24 15:22:39 through
  2026-07-25 05:47:46 UTC;
- Qwen 64K served four successful Chat Completions before an explicit SIGTERM
  switch to Nemotron. Its recorded `EngineDeadError` occurred during shutdown,
  not as an independent model crash; and
- the benchmark repository was not present on DGX, while a TeaQL Agent Kit
  checkout, a non-modeling smoke build, DGX-local MiMoCode state, and a
  DGX-local launcher were present.

The 10,241 HTTP 400 responses closely match 10,226 blank-finish assistant
messages in the two runaway checkpoint-writer sessions plus 15 earlier 400
responses. This is strong evidence that MiMoCode's automatic checkpoint path,
not TeaQL compilation, caused the high-frequency failure period. Request counts
must not be interpreted as benchmark-round counts because they include probes,
retries, subrequests, and potentially unknown callers.

## MiMoCode failure analysis

### What was proven to work

- MiMoCode 0.1.7 connected to the DGX OpenAI-compatible service.
- Chat Completions and structured tool calls worked.
- The model successfully requested a local file read and a local shell command.
- Local tools ran on the ARM64 developer workstation while inference ran on
  DGX Spark.

### What did not work reliably

- The 32K service context was too small for MiMoCode's approximately 26.7K-token
  base prompt and tool schema; the service was increased to 64K.
- The headless client could emit `reason=stop` but remain alive.
- The client could invoke `skill_search` despite task instructions.
- Round 3 failed pass@1, hallucinated Rust accessors, omitted a required trait
  import, assumed an unsupported log display API, and required runtime-guided
  correction of a persisted numeric value.
- Round 4 exhausted the 64K context without changing the model. Its recovery
  session repeatedly inspected directories and did not execute the requested
  compact transform.
- Both the Round 3 and Round 4 records show unnecessary re-reading or command
  repetition, including retries after a known local proxy failure.

### Later DGX-local MiMoCode failure

MiMoCode 0.1.9 was installed on DGX and launched against
`http://localhost:30704/v1`. Its main session contained 10 user messages,
142 assistant messages, 137 tool parts, and five checkpoint records.

The main session spawned three checkpoint-writer sessions. One was
normal-sized. Two became runaway loops:

- 3,420 assistant messages, including 3,387 with no finish reason, over an
  approximately 11-minute failure window; and
- 6,864 assistant messages, including 6,839 with no finish reason, over an
  approximately 25-minute failure window.

For the second loop, the preserved error is exact: MiMoCode requested 8,192
output tokens while its prompt contained at least 57,345 input tokens. The
minimum total was therefore 65,537, exceeding the service limit by one token.
The processor logged this same context error 847 times between 05:44:14 and
05:48:03 UTC, while the checkpoint subagent continued creating blank assistant
messages. This explains both the apparent lack of execution progress and the
roughly 200 MB of rotated client logs.

### Correct interpretation

MiMoCode's **provider and local-tool integration passed**, but its **autonomous
completion reliability failed the larger benchmark requirement**. The large
successful benchmark artifacts should primarily be attributed to direct Python
orchestration, deterministic TeaQL validation, local transformation scripts,
and Cargo repair loops—not to unattended MiMoCode completion.

## Script inventory by function

| Function | Principal scripts |
|---|---|
| MiMoCode launcher | `scripts/mimo-dgx` |
| Round 2 API parameter matrix and feedback repair | `scripts/run_modeling_matrix.py` |
| Large-model preparation and validation | `scripts/prepare_round4_model.py`, `scripts/validate_round4_model.py`, `scripts/prepare_round5_model.py`, `scripts/validate_round5_model.py` |
| Monolithic Qwen/Nemotron generation | `scripts/run_round8_model.py`, `scripts/run_round9_model.py`, `scripts/run_round10.py`, `scripts/run_round11.py`, `scripts/run_round12.py`, `scripts/run_round13.py` |
| Modular Nemotron generation | `scripts/dgx_round10_agent.py`, `scripts/dgx_round11_agent.py`, `scripts/dgx_round14_agent.py` |
| Structural cleanup and repair | `scripts/fix_xml.py`, `scripts/fix_round8_xml.py`, `scripts/transform_to_ksml.py` through `transform_to_ksml_v4.py`, `scripts/remove_ids.py`, `scripts/remove_names.py`, `scripts/inject_gender_type.py` |
| Phase 2 scale orchestration | `scripts/run_round.py`, `scripts/run_all.sh`, `scripts/monitor.sh` |
| Local TeaQL/Cargo execution | `scripts/run_round5_model.py`, `scripts/run_round6_model.py`, `scripts/run_round7_model.py`, `scripts/run_native_round.py` |

## Evidence gaps and cautions

1. Raw workstation MiMoCode JSONL transcripts are not checked in. DGX-local
   MiMoCode logs and its SQLite database still exist remotely and were
   inspected through aggregates, but were not copied because they contain full
   prompts and private session content.
2. Phase 2 success reports do not preserve per-request duration, token usage,
   retry counts, raw model response, evaluation output, or compile output.
   Generated outputs and Git commits prove final artifact completion more
   strongly than pass@1 quality.
3. Several scripts contain a literal unauthenticated endpoint address. The
   reports also record that the model API was bound on all interfaces. Future
   evidence should use a redacted environment variable and authenticated or
   source-restricted access.
4. Round numbers are reused for original monolithic tests and later modular
   reruns. Any comparison must identify both the round and architecture.
5. Some later executive reports compress or relabel round numbering. The
   Phase 2 sequence supported by the actual per-round reports is:
   R15=10, R16=20, R17=30, R18=40, R19=50, R20=60, R21=70, R22=80,
   R23=90, R24=100, R25=120, R26=140, and R27=160 objects.
6. An earlier configured SSH target was unrelated (`legal-letter`, `x86_64`).
   The subsequently supplied DGX SSH endpoint was verified as `spark-007`,
   `aarch64`, NVIDIA GB10. Only the latter is used for remote evidence.

## Recommended record format for the next run

For each invocation, preserve:

- run ID, start/end timestamps, client and version;
- model, server/container version, context, output cap, and concurrency;
- redacted endpoint identity and request ID;
- exact script command and Git commit;
- raw model response or MiMoCode JSONL;
- tool calls, exit reasons, timeout/watchdog action, and retry count;
- pre-repair and post-repair TeaQL evaluation;
- Cargo check/test/run output and return codes;
- hashes of generated artifacts; and
- DGX system/GPU snapshots before and after the run.
