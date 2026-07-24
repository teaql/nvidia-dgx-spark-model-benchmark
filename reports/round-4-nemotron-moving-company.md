# Round 4: 223-Object Moving-Company Rust TeaQL Q/E Benchmark

## Executive result

Round 4 implements a moving-company backend rather than extending the prior
school example. The final semantic model contains 223 direct objects across 14
English workbenches. TeaQL generated the Rust domain and console workspace,
which were compiled, tested, and executed against a real SQLite database.

The final outcome is a repaired pass:

- Fixed structural validator: passed with zero errors.
- Official TeaQL evaluate: 0 errors, 16 warnings, 119 suggestions, 881 solids.
- Rust formatting, checking, testing, and execution: passed.
- Automated tests: 10 passed, 0 failed.
- Sample generation: 162 business entity types, 0 skipped.
- Q APIs: five workbenches, 20 returned rows each.
- E expressions: two safe chains, 20 successful values each.
- Final isolated SQL run: 74 SELECT and 36,900 INSERT statements.

This is large enough to represent a substantial enterprise monolithic backend,
not a toy CRUD example.

## Domain scope

The model preserves the frozen definitions of `platform`, `merchant`, and
`employee` and covers:

- Operations & Logistics
- Employees & Payroll
- Customer Management
- Products & Services
- Marketing & Sales
- Finance & Accounting
- Asset Management
- Administration & Compliance
- Identity & Access
- Activity & Audit
- Notifications & Automation
- API & Integrations
- Platform Administration
- Organization Administration

The canonical 155-object moving-company seed was expanded to 223 objects.
Every added object has a meaningful domain name; the validator rejects numeric
count-padding names.

## Modeling execution

The first Nemotron session loaded the complete rule set, fixed task, and large
model file. It then exceeded the 65,536-token context window after reserving
8,192 output tokens and made no model change.

A recovery session was explicitly instructed to use a compact XML-transform
script. It repeatedly listed directories and never entered the transformation
step. Modeling pass@1 therefore failed, and the recovery agent did not complete
the model.

A deterministic migration script then classified the seed objects, replaced
the frozen definitions, added required anchors, assigned English workbench
metadata, and normalized tenancy. The fixed validator first exposed an
insufficient pool of compliance objects. After that was corrected, the first
official TeaQL evaluation reported three unresolved references. Replacing the
`remote_ip()` and `cityByIp()` example helpers with concrete documentation
values reduced the official evaluation to zero errors.

This result highlights the principal scaling constraint: domain-model size and
relationship density place more pressure on agent context than Rust generation
or focused Q/E implementation.

## Rust generation and assist usage

The workflow strictly followed this order:

1. Generate `rust-lib-core`.
2. Generate `rust-app-console`.
3. Read the generated `AGENTS.md` in full.
4. Fetch object-specific query, create, and expression assists plus runtime and
   debug assists.
5. Write customer-owned Rust code and compile it.

No generated domain file was manually edited.

## Runtime Q results

| Workbench | Object | Rows |
|---|---|---:|
| Operations & Logistics | `move_order` | 20 |
| Customer Management | `customer` | 20 |
| Employees & Payroll | `employee` | 20 |
| Finance & Accounting | `invoice` | 20 |
| Asset Management | `vehicle` | 20 |

Every `execute_for_list()` invocation carries a business-specific `.comment()`
and `.purpose()`.

The two runtime E expressions were:

- `E::customer(customer).get_name().eval()`: 20 non-null values.
- `E::invoice(invoice).get_invoice_number().eval()`: 20 non-null values.

## SQL evidence

TeaQL's default file sink captured the complete SQL stream. The final isolated
successful run contained:

- SELECT: 74
- INSERT: 36,900
- UPDATE: 0
- DELETE: 0

Five representative application queries and one representative mutation are
preserved in [`sql-evidence.md`](../artifacts/round-4/sql-evidence.md). The
53 MB transient raw log was intentionally excluded from the repository.

## Findings

### Dynamic runtime/debug assist differs from runtime 4.1.1

- The guide says `TEAQL_AUDIT=production`; the runtime accepts
  `_silent`, `_summary`, or `_full`.
- The guide says `TEAQL_SCHEMA=execute`; the runtime accepts
  `_verify`, `_dryrun`, or `_execute`.
- The debug guide names `TEAQL_SQL_LOG`; startup rejects it and suggests
  `TEAQL_SQL`.
- `UserContext.sql_logs()` returned no entries even though the default file
  sink captured the statements.

The first two runtime attempts stopped during environment validation and did
not contaminate the database. Using the values reported by the runtime led to
a successful run.

### Rust customer code required two small repairs

The first compile showed that `SqlLogEntry` is not a string and its `.sql`
field must be read. Test compilation then showed that enabling SQL logging
requires a mutable runtime. All ten tests passed after these corrections.

### Non-blocking model warnings

The 16 TeaQL warnings primarily concern audit masking for privacy fields,
constant-module locality, and two generic seed values. The fixed `employee`
definition could not be modified, so its privacy warnings are recorded as
future hardening rather than silently changing the benchmark contract.

## DGX end snapshot

Collected at 2026-07-24 09:38 UTC:

- GPU: NVIDIA GB10, driver 580.159.03.
- GPU utilization: 0%; temperature: 46°C; power: 11.89 W.
- System memory: 130.66 GB total, 99.11 GB used, 31.55 GB available.
- Swap used: 4.28 GB.
- Load average: 0.14 / 0.20 / 0.24.
- Model container: running, 1.42% CPU, 8.2 GiB / 121.7 GiB.
- Served model: `nemotron-3-super`, maximum context 65,536.

The model endpoint remains bound to `0.0.0.0:30703` without an API key.
Production deployment should restrict source addresses or add an
authenticated reverse proxy.

## Artifacts

- [Complete Round 4 source tree](../artifacts/round-4/source-evidence/)
- [Archived Round 3 source tree](../artifacts/round-3/source-evidence/)
- [Machine-readable summary](../artifacts/round-4/summary.json)
- [Fixed validator](../scripts/validate_round4_model.py)
- [Deterministic migration script](../scripts/prepare_round4_model.py)
