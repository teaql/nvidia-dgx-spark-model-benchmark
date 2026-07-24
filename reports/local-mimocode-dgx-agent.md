# Local MiMoCode with a DGX Spark Model Server

> Date: 2026-07-24
> Client: MiMoCode 0.1.7 on the developer workstation
> Model server: Nemotron 3 Super 120B NVFP4 on DGX Spark
> Data: [local-agent-smoke.json](../artifacts/round-2/local-agent-smoke.json)

## Result

The architecture in which DGX performs model inference while the agent, file
tools, and compiler run locally was successfully validated.

- MiMoCode was installed on the workstation through Homebrew.
- A custom OpenAI-compatible provider was configured as
  `dgx/nemotron-3-super`.
- MiMoCode successfully used Chat Completions and structured tool calls.
- `read` and `bash` tools executed in the local `teaql-agent-kit` directory.
- Local Cargo was available as `1.99.0-nightly`.
- The smoke test did not modify the TeaQL worktree.

## Final model-service configuration

| Item | Value |
|---|---|
| Active container | `nemotron-super-agent-server` |
| Port | 30703 |
| Served model | `nemotron-3-super` |
| Context | 65,536 tokens |
| Maximum concurrent sequences | 1 |
| KV cache | 8 GiB / 338,602 tokens |
| Prefix caching | Enabled |
| Chunked prefill | Enabled |
| Thinking by default | Disabled |
| Tool parser | `qwen3_coder` |
| Reasoning parser | `super_v3` |
| Cache | Persisted on the host |

The previous `nemotron-super-server` container remains stopped as a rollback
option; its former port is not active.

## Why the context was increased from 32K to 64K

MiMoCode's base system prompt and tool schema consume approximately 26.7K
tokens. A 32K context approached the limit after a single tool call and
triggered checkpoint or compaction continuation. The same read-only task no
longer triggered that behavior at 64K.

## Why KV cache was limited

Automatic allocation reserved 39.02 GiB of KV cache and left only about
1.81 GiB of available system memory, with approximately 6.82 GiB of swap in
use. An explicit 8 GiB cache left roughly 32–34 GiB available, providing a
safer operating margin for the OS and model service.

Eight GiB still holds 338,602 tokens. At one concurrent 64K request, VLLM
reported approximately 5.17 times the required theoretical capacity.

## Prefix and compilation caches

MiMoCode carries a long, mostly stable agent prompt, making it suitable for
prefix caching. Repeated tests reached an observed prefix-cache hit rate of
80.6%. Mamba prefix caching remains experimental in the tested VLLM nightly and
should be monitored on long tasks.

Host persistence was also configured for compilation and autotuning caches:

- Torch compilation fell from about 21.7 seconds to 3.76 seconds.
- All 170 FlashInfer autotune configurations hit the cache.
- Model weights still required about 6.4 minutes to reload.

## MiMoCode configuration

The private configuration is stored at:

```text
~/.config/mimocode/mimocode.jsonc
```

It has mode `0600` and is not committed. It defines:

- provider: `dgx`
- adapter: `@ai-sdk/openai-compatible`
- context: 65,536
- maximum output: 8,192
- tool calls: enabled
- reasoning: disabled

The repository supplies this launcher:

```bash
scripts/mimo-dgx /Users/Philip/githome/teaql-agent-kit
```

Headless example:

```bash
scripts/mimo-dgx run \
  --pure \
  --dir /Users/Philip/githome/teaql-agent-kit \
  "Inspect this project and run the necessary local tests."
```

The wrapper sets `MIMOCODE_DISABLE_BUILTIN_SKILLS=true` to reduce built-in
skill injection that is unsuitable for a 64K local model. MiMoCode can still
discover external skills from the user directory.

## Verified local tools

The model issued a `read` call against the local
`/Users/Philip/githome/teaql-agent-kit/AGENTS.md` and returned its first
heading correctly.

It also issued a local shell call for `pwd`, architecture, and Cargo version:

```text
/Users/Philip/githome/teaql-agent-kit
arm64
cargo 1.99.0-nightly (2f0e7011e 2026-07-05)
```

This demonstrates that inference ran on DGX while file access and compilation
remained on the ARM64 Mac.

## Remaining issues

1. MiMoCode 0.1.7 sometimes invokes `skill_search` despite being told not to.
2. Headless `mimo run` can remain alive after emitting `reason=stop`; an outer
   timeout or watchdog is required.
3. Port 30703 is bound to all interfaces and VLLM has no API key. Restrict
   sources with a firewall or configure authenticated access.
4. FP8 KV cache reports uncalibrated q/prob scales; compare quality against
   BF16 KV.
5. The service is single-concurrency. Multiple agents will queue and require a
   separate throughput and memory benchmark.

## Recommended operation

- Interactive work: `scripts/mimo-dgx <project-directory>`.
- Automated work: set an explicit timeout and terminate after parsing
  `reason=stop`.
- Keep thinking disabled for modeling and code tasks; use deterministic
  validation, compilation, and tests as the repair loop.
- Run heavy builds locally. Do not install project toolchains or mount source
  code on DGX.

This report and its machine-readable data contain no remote address, SSH
credential, or proxy secret.
