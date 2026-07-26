# NVIDIA DGX Spark Model Benchmark

This repository evaluates whether local models running on NVIDIA DGX Spark can
complete TeaQL Agent Kit engineering tasks, with coding agents, file tools,
compilation, tests, and databases running on a separate developer workstation.

The benchmark progresses from API and modeling smoke tests to a 223-object
moving-company backend. It is designed as evidence for private-network AI
coding: model inference stays on DGX Spark while source code and build tools
remain inside the customer's engineering environment.

## Agent design

- [DGX Spark Ratatui coding-agent design](docs/DGX-SPARK-RATATUI-CODING-AGENT-DESIGN.md)

## Benchmark artifacts

- [DGX Spark execution ledger and MiMoCode failure analysis](reports/DGX-SPARK-EXECUTION-LEDGER.md)
- [DGX remote verification snapshot](artifacts/dgx-remote-verification-20260726.json)
- [Bounded MiMoCode simple-model test](artifacts/simple-model-test-20260727/summary.json)
- [Bounded modeling A/B refinement](artifacts/simple-model-test-20260727/ab-summary.json)
- [Bounded direct Python client simulation](artifacts/python-client-simulation-20260727/summary.json)
- [Bounded generate-validate-repair pipeline](reports/bounded-model-pipeline.md)
- [Fresh pipeline run](artifacts/bounded-model-pipeline-20260727/summary.json)
- [Failed-candidate repair run](artifacts/bounded-model-repair-test-20260727/summary.json)
- [Moving-company 30-object pipeline run](reports/moving-company-30-pipeline.md)
- [Moving-company 30-object final model](artifacts/moving-company-30-pipeline-20260727/final-model.xml)
- [Round 1: DGX Spark and TeaQL feasibility report](reports/round-1-dgx-spark-teaql.md)
- [Round 1 machine-readable system snapshot](artifacts/round-1/system-snapshot.json)
- [Round 2: Nemotron 3 Super 120B modeling report](reports/round-2-nemotron-super-modeling.md)
- [Round 2 parameter-matrix summary](artifacts/round-2/modeling-runs/summary.json)
- [Round 2 system snapshot](artifacts/round-2/system-snapshot.json)
- [Local MiMoCode and DGX model-server integration](reports/local-mimocode-dgx-agent.md)
- [Local agent smoke-test data](artifacts/round-2/local-agent-smoke.json)
- [Round 3: Nemotron agent and Rust TeaQL Q/E report](reports/round-3-nemotron-agent-rust-teaql.md)
- [Round 3 machine-readable summary](artifacts/round-3/summary.json)
- [Round 3 complete TeaQL source evidence](artifacts/round-3/source-evidence/)
- [Round 4 preparation and acceptance criteria](reports/round-4-preparation.md)
- [Round 4 fixed moving-company task](benchmarks/round-4/moving-company-180-object-task.md)
- [Round 4: 223-object moving-company Rust TeaQL report](reports/round-4-nemotron-moving-company.md)
- [Round 4 machine-readable summary](artifacts/round-4/summary.json)
- [Round 4 SQL evidence](artifacts/round-4/sql-evidence.md)
- [Round 4 complete TeaQL source evidence](artifacts/round-4/source-evidence/)

## Evidence policy

All reader-facing documentation is maintained in English. Raw model responses
remain unchanged when translation would alter benchmark provenance. Generated
TeaQL source is also kept byte-for-byte as produced by the generator.
