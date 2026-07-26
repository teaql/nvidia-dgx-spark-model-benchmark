# Moving-Company 30-Object Pipeline Run

## Current outcome

The bounded Python pipeline produced a final 30-object moving-company platform
model on DGX Spark. The final artifact passed the exact local acceptance
specification with zero errors and zero warnings.

After explicit export authorization, the official TeaQL evaluator also passed
the model with 0 errors, 141 warnings, 6 suggestions, and 67 solids. All 34
relationships resolved successfully.

## Final module distribution

| Module | Objects |
|---|---:|
| Operations and Logistics | 5 |
| Employees and Payroll | 5 |
| Customer Management | 4 |
| Products and Services | 4 |
| Marketing and Sales | 3 |
| Finance and Accounting | 3 |
| Asset Management | 3 |
| Platform, Access, and Audit | 3 |
| **Total** | **30** |

The model contains 34 resolved object relationships and no unresolved local
relationship targets.

## Official TeaQL evaluation

The 141 warnings consist of:

- 138 recommendations to use concrete typical values instead of the
  deliberately restricted `string()` value form
- One root-name recommendation to end the service name in `-service`
- One warning about six independent domain roots
- One privacy warning recommending audit masking for
  `customer.billing_address`

The six suggestions recommend marking `work_log` and `audit_log` as logging
objects and adding four more personal fields to audit masking.

These findings affect model richness and privacy metadata, not XML structure or
reference validity.

## Pipeline refinement

The preliminary candidate was structurally complete but grouped `address`,
`job_assignment`, and `work_log` under inappropriate modules. The acceptance
specification was strengthened to validate module ownership as well as exact
objects, fields, relationships, version fields, and soft-delete fields.

The pipeline then:

1. Rejected the candidate with seven deterministic domain-acceptance errors.
2. Started one fresh, stateless DGX repair request.
3. Used 3,981 prompt tokens and 2,027 completion tokens.
4. Finished the repair request in 127.399 seconds with `finish_reason=stop`.
5. Produced a final model with 30 objects and no local acceptance errors.

## Scope note

To remain compatible with the verified KSML value-form whitelist, financial,
date, duration, and quantity fields currently use `string()` rather than richer
types. Versioning, soft deletion, magic-link authentication, RBAC, audit
masking, activity logging, insurance and contract document references, and
automation hooks are represented in the semantic model, but their runtime
behavior still requires generated-code and integration testing.

## Evidence

- `benchmarks/moving-company-30/task.md`
- `benchmarks/moving-company-30/acceptance.json`
- `artifacts/moving-company-30-pipeline-20260727/final-model.xml`
- `artifacts/moving-company-30-pipeline-20260727/summary.json`
- `artifacts/moving-company-30-pipeline-20260727/teaql-evaluate.md`
