# Round 3: DGX Spark, Nemotron 3 Super 120B, and Rust TeaQL

> Date: 2026-07-24
> Stage: MiMoCode agent, TeaQL Rust generation, and compiled Q/E execution
> Target: `autonomous` branch of `teaql-vibe-kit`
> Data: [summary.json](../artifacts/round-3/summary.json)

## Result

Round 3 completed this end-to-end loop:

```text
Natural-language scenario
→ KSML model
→ TeaQL evaluate
→ model review gate
→ Rust core and console generation
→ model-specific Q/E/create assist
→ customer-owned Rust code
→ cargo check, test, and run
→ SQLite and SQL-log evidence
```

The final outcome was a repaired pass:

- Model evaluation: 0 errors, 0 warnings, 1 suggestion.
- `cargo check`: passed.
- `cargo test`: one Q/E SQLite integration test passed.
- `cargo run`: passed.
- One Primary School row was returned and E expressions produced the expected
  values.
- Nine SQL-log entries were captured.

Nemotron read the rules, modeled the scenario, invoked assists, and produced a
reasonable first Rust implementation, but pass@1 failed. The first version had
eight compiler errors. After compilation succeeded, runtime evidence showed
that the string value `"320"` persisted as numeric zero. Compiler- and
runtime-guided repair produced the final pass.

## Scenario and model

The model contains three concepts:

- `Platform`: the single domain root and platform tenancy boundary.
- `School`: name, address, principal, student count, Platform, and School Type.
- `School Type`: a constant object with `PRIMARY=1001` and `SECONDARY=1002`.

The model review status is `confirmed_with_assumptions`. The workflow stopped
when it detected that the final model fields differed from the first review
summary, synchronized the review to the actual model, and ran evaluate again.
This shows that a review gate must compare content rather than merely checking
a status string.

## TeaQL toolchain

| Item | Result |
|---|---|
| cargo-teaql | 2.0.8, exact required version |
| Evaluate | 0 errors / 0 warnings / 1 suggestion / 15 solids |
| Rust library | Generated `rust-lib-core`, treated as read-only |
| Rust app | Generated `rust-app-console` with local `AGENTS.md` |
| Runtime | TeaQL 4.1.1 with SQLite |
| Rust toolchain | 1.99.0 nightly |

Dynamic assists read during the run:

- `rust-assist-query/school`
- `rust-assist-query/platform`
- `rust-assist-create/school`
- `rust-assist-expression/school`
- `rust-assist-runtime-custom`
- `rust-assist-debug/school`

The generator initially resolved an output path relative to
`--cwd app-playground`, creating a duplicate directory layer. The two complete
generated directories were moved intact to the canonical location; generated
files were not edited.

## Agent behavior

### What worked

- Read the repository rules, playbook, and generated `AGENTS.md`.
- Followed the model-first and review-first workflow.
- Invoked runtime, query, create, expression, and debug assists.
- Included `.purpose()`, `.comment()`, and `.audit_as()` intent guardrails.
- Designed a coherent Platform → Primary School → Q filter → E extraction
  scenario.

### Pass@1 failures

The first Rust version produced eight errors:

| Pattern | Observation |
|---|---|
| Accessor hallucination | `get_name`, `get_address`, `get_student_count`, `get_id` |
| Missing trait import | `.audit_as()` could not find the `Entity` trait |
| Log-type assumption | `SqlLogEntry` was treated as `Display` |

Invoking an assist did not guarantee strict adherence to it. The model
incorrectly transferred E-expression `get_*` conventions to entity accessors.

### Runtime semantic failure

The first repair passed the student count as `"320"`. It compiled but generated
SQL with `student_count=0`. Supplying numeric `320` produced the correct data.
This defect required actual execution and database assertions; `cargo check`
alone could not expose it.

### Agent efficiency

MiMo headless did not exit after completion and required external termination.
The agent repeatedly listed directories, re-read rules, and retried Cargo
commands after a known proxy failure. Final visible context was approximately
45K tokens for the Rust stage and 49K for modeling. These are last-observed
context sizes, not exact billed-token totals.

## Final Q/E evidence

```text
Q/E scenario passed: 1 Primary school row, 1 Primary type row,
school = "Sunshine Primary School",
platform = "Standard Education Platform",
type = "PRIMARY", platform_id = 1, students = 320,
9 SQL log entries
```

Representative SQL:

```sql
INSERT INTO school_data (..., student_count, ..., platform, school_type)
VALUES (..., 320, ..., 1, 1001);

SELECT id, version, name, address, principal, student_count
FROM school_data
WHERE ((version > 0) AND (school_type = 1001))
LIMIT 20;
```

The supplementary test added `Q::school_types_minimal()` and loaded Platform
and School Type through the School query. This relationship expression compiled
and ran successfully on its first attempt:

```rust
E::school(school)
    .get_school_type()
    .get_code()
    .eval()
```

Three explicit Q requests plus relation loads produced eight selects and one
insert. The Primary School query returned one row in 65 microseconds.

## Verification matrix

| Check | Result |
|---|---|
| TeaQL evaluate | PASS |
| Generated app `AGENTS.md` | PASS |
| cargo fmt | PASS |
| cargo check | PASS |
| cargo test | PASS, one integration test |
| cargo run | PASS |
| Q constant filter | PASS |
| E scalar extraction | PASS, five values |
| E relationship chain | PASS |
| Audit/comment/purpose guardrails | PASS |
| SQL capture | PASS, nine entries |
| Nemotron pass@1 | FAIL |
| Compiler/runtime repaired result | PASS |

The workstation's global Cargo proxy pointed to unavailable
`127.0.0.1:1087`. Commands used a temporary per-process proxy override and did
not modify global configuration.

## DGX end snapshot

| Item | Value |
|---|---|
| GPU | NVIDIA GB10 |
| Utilization | 0% at snapshot |
| Temperature | 45°C |
| Power | 11.93 W |
| System memory | 121 GiB total / 91 GiB used / 30 GiB available |
| Swap | 15 GiB total / 4.2 GiB used |
| Load average | 0.10 / 0.32 / 0.46 |
| Model container | Up 53 minutes |
| Container memory | 7.603 GiB, incomplete under unified memory |
| Service mapping | 30703 to container 8000 |

The endpoint was still published on all addresses without an API key. An
internal production server should restrict source networks or use an
authenticated reverse proxy.

## Recommendations

1. Score pass@1 and repaired pass independently.
2. Save every MiMo JSONL run and measure elapsed time, tool calls, errors, and
   repair iterations.
3. Require entity accessors to come from assists or compiler feedback; never
   transfer E-expression conventions by analogy.
4. Assert persisted numeric, enum, and foreign-key values after execution.
5. Change strategy after one repeated network failure.
6. Add automatic headless exit or a watchdog.
7. Restrict and authenticate port 30703.

## Artifacts

- [Complete archived source tree](../artifacts/round-3/source-evidence/)
- [Machine-readable summary](../artifacts/round-3/summary.json)

No artifact contains SSH credentials, proxy credentials, or the remote host
address.
