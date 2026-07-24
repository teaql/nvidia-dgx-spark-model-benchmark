# Round 5 Preparation

> Scenario: Moving Company Management Platform
> Target: at least 180 TeaQL objects with real Rust Q/E execution
> Status: fixed task, preserved-object baseline, and structural validator ready

## Frozen inputs

- Fixed task:
  [`moving-company-180-object-task.md`](../benchmarks/round-5/moving-company-180-object-task.md)
- `platform`, `merchant`, and `employee` baseline:
  [`preserved-core-objects.xml`](../benchmarks/round-5/preserved-core-objects.xml)
- Fixed structural validator:
  [`validate_round5_model.py`](../scripts/validate_round5_model.py)

## Compatibility decision

The three core objects in the legacy moving-company example used underscore
module keys, which conflict with the current lowercase kebab-case rule in
`teaql-vibe-kit`. Round 5 preserves their fields, relationships, and tenancy
semantics while freezing current-rule-compatible English workbench metadata:

- `platform`: `Platform Administration / platform-administration`
- `merchant`: `Organization Administration / organization-administration`
- `employee`: `Employees & Payroll / employees-payroll`

The model is not allowed to add, remove, or rename attributes on these three
frozen objects.

## Object and workbench thresholds

The fixed task defines 14 English workbenches whose minimum object counts total
exactly 180. The validator checks:

- `_name`, `_module`, and `_module_key` on every object;
- exact English workbench names and kebab-case keys;
- membership in the allowed workbench set;
- every per-workbench minimum;
- 93 required domain anchor objects in their correct workbenches;
- attribute-for-attribute equality of the three preserved objects;
- rejection of count-padding names such as `entity_001` and `object_42`;
- fixed root model name and alias.

This validator supplements the task-specific contract. The model must still
pass the official `cargo teaql --input <model> evaluate` command.

## Round 5 success criteria

1. At least 180 unique direct-child objects.
2. Fixed structural validation passes.
3. TeaQL evaluate reports zero errors.
4. The model review gate passes.
5. `rust-lib-core` and `rust-app-console` are generated in that order.
6. `cargo fmt --check`, `cargo check`, `cargo test`, and `cargo run` pass.
7. At least ten automated tests pass.
8. Q APIs from at least five workbenches execute against a database.
9. At least two E chains execute against runtime-loaded entities.
10. SQL evidence contains both mutation and select statements.
11. Model and Rust pass@1 and repaired-pass results are reported separately.

## Recommended execution sequence

Use one bounded session for KSML and `MODEL_REVIEW.md`, a second for generation,
and a third for Q/E implementation. Preserve raw MiMo JSONL, terminal output,
elapsed time, token observations, system snapshots, and compiler errors at each
stage. This avoids losing measurements when a headless client fails to exit or
when an agent repeatedly reloads long documentation.
