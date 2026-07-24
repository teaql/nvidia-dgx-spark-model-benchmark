# Phase 2 Benchmark Test Plan: Moving Company (Scale 10 -> 200)

## Overview
This document outlines the execution plan for Phase 2 of the DGX Spark Model Benchmarking. We are initiating a massive, 15-round scaling test based on the **Moving Company** domain. This phase will strictly utilize the **latest modeling prompt standards** (from `teaql-agent-kit/prompts/modeling`) and enforce the `<_include>` modular architecture for models exceeding 20 objects.

## Objective
To definitively map the performance, stability, and code-generation reliability of the DGX Nemotron model as the complexity of the domain scales from a microservice (10 objects) up to a massive enterprise monolith (200 objects).

## Scale Progression (15 Rounds)
- Round 15: 10 Objects
- Round 16: 20 Objects
- Round 17: 30 Objects
- Round 18: 40 Objects
- Round 19: 50 Objects
- Round 20: 60 Objects
- Round 21: 70 Objects
- Round 22: 80 Objects
- Round 23: 90 Objects
- Round 24: 100 Objects
- Round 25: 120 Objects
- Round 26: 140 Objects
- Round 27: 160 Objects
- Round 28: 180 Objects
- Round 29: 200 Objects

*(Note: Round numbering continues from Phase 1 to avoid directory conflicts).*

## Standard Execution Loop for Each Round
For every single scale point listed above, the Agent will execute the following automated loop:

1. **Modeling & Evaluation**:
   - Utilize a multi-threaded Python agent (similar to `scripts/dgx_round14_agent.py`) to hit the DGX Spark endpoint.
   - For scales > 20 objects, automatically partition the domain into modules using `main.xml` and `<_include>`.
   - Run `cargo teaql evaluate` until a **0 Error** state is achieved.

2. **Code Generation & Testing**:
   - Run `cargo teaql generate` to unpack the Rust Core and App Console.
   - Run `cargo check` to pre-compile the generated Rust microservices, guaranteeing there are no Type or Trait Bound errors (such as the `IdentifiableEntity` bug, which is now resolved by omitting explicit `id` attributes).
   - Write actual Rust testing code asserting the behavior of the newly generated entities.

3. **Log & Telemetry Verification**:
   - Capture runtime logs and track any compilation bottlenecks.

4. **Reporting (The Final Deliverable)**:
   - Generate a dedicated `Round-X-Report.md` in the `reports/` directory.
   - **Environment Matrix**: Each report must log the exact versions (Agent, Rust/Java Runtime, teaql-code-gen API, GPU Hardware, CUDA, and LLM Context Window limits).
   - **Token Economy Analysis**: Each report MUST contain a section analyzing **"How the TEAQL Engineering Suite Saves Tokens"**. This section will document the exact token consumption, highlighting how the modular `<_include>` architecture prevents context bloat and drastically reduces token overhead compared to monolithic XML generation.

## Execution Strategy
Due to the massive scale of this 15-round generation loop (requiring hundreds of LLM calls, extensive Rust compilation, and reporting), this test suite will be executed sequentially as a long-running, unsupervised overnight task.
