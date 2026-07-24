# Round 4 Fixed Task — Moving Company Management Platform

## Objective

Build a comprehensive, multi-tenant moving-company management platform with a
semantic TeaQL KSML model and Rust runtime.

Work model-first:

1. Create the semantic KSML model.
2. Run `cargo teaql --input <model> evaluate`.
3. Repair until evaluation has zero errors.
4. Review the model and record `confirmed_with_assumptions`.
5. Generate `rust-lib-core`, then `rust-app-console`.
6. Use model-specific `rust-assist-*` output before writing customer code.
7. Compile, test Q and E APIs against a real local database, and write a run
   report.

## Required Architecture

- Platform-managed multi-tenancy.
- `platform` is the single domain root.
- `merchant` is the tenant owner and references `platform()`.
- Tenant-owned business objects use `merchant="merchant(context)"`.
- Preserve the exact `platform`, `merchant`, and `employee` definitions from
  [`preserved-core-objects.xml`](preserved-core-objects.xml). Do not add,
  remove, or rename attributes on those three objects.
- All objects are direct children of one `<root>`.
- At least 180 unique direct-child objects.
- Objects must be domain-specific; names such as `entity_001`, `object_42`, or
  other count-padding placeholders are forbidden.

## Workbench Metadata

Every object must define:

- `_name`: English display name.
- `_module`: one of the English workbench names below.
- `_module_key`: the corresponding lowercase kebab-case key.

| `_module_key` | `_module` | Minimum objects |
|---|---|---:|
| `platform-administration` | `Platform Administration` | 3 |
| `organization-administration` | `Organization Administration` | 3 |
| `operations-logistics` | `Operations & Logistics` | 25 |
| `employees-payroll` | `Employees & Payroll` | 22 |
| `customer-management` | `Customer Management` | 16 |
| `products-services` | `Products & Services` | 18 |
| `marketing-sales` | `Marketing & Sales` | 15 |
| `finance-accounting` | `Finance & Accounting` | 17 |
| `asset-management` | `Asset Management` | 17 |
| `administration-compliance` | `Administration & Compliance` | 14 |
| `identity-access` | `Identity & Access` | 10 |
| `activity-audit` | `Activity & Audit` | 6 |
| `notifications-automation` | `Notifications & Automation` | 7 |
| `api-integrations` | `API & Integrations` | 7 |

The minimums total 180. Constant objects count toward the workbench minimum
when they are meaningful finite business sets.

## Core Modules

### Operations & Logistics

Cover moves, quotes, routes, route stops, time slots, fulfillment events,
addresses, crews, dispatch, inventory handling, damage reporting, proof of
delivery, and operational exceptions.

Required anchors:

`move_order`, `move_quote`, `route`, `route_stop`, `time_slot`,
`fulfillment_event`, `address`, `crew`, `dispatch_assignment`,
`damage_report`, `proof_of_delivery`.

### Employees & Payroll

Cover staff registry, departments, job assignments, shifts, worked hours,
payroll calculations, payslips, bonuses, deductions, leave, certification,
training, and availability.

Required anchors:

`employee`, `department`, `job_assignment`, `work_shift`, `worked_hours`,
`payroll_period`, `payroll_calculation`, `payslip`, `bonus`,
`leave_request`, `employee_certification`.

### Customer Management

Cover private and corporate customers, linked contacts, customer addresses,
billing profiles, preferences, consent, service history, complaints, and
customer notes.

Required anchors:

`customer`, `private_customer_profile`, `corporate_customer_profile`,
`customer_contact`, `billing_profile`, `customer_history`,
`customer_preference`, `customer_consent`.

### Products & Services

Cover moving, cleaning, box rentals, packing, storage, disposal, and additional
services, including configuration, pricing, bundles, availability, rental
periods, and service areas.

Required anchors:

`product`, `service`, `moving_service`, `cleaning_service`, `box_rental`,
`service_configuration`, `price_list`, `service_price`, `service_bundle`.

### Marketing & Sales

Cover campaigns, audiences, channels, discount codes, leads, opportunities,
lead activities, attribution, conversion events, funnels, and conversion
metrics.

Required anchors:

`campaign`, `discount_code`, `lead`, `sales_opportunity`,
`lead_activity`, `conversion_event`, `conversion_metric`.

### Finance & Accounting

Cover payments, invoices, invoice lines, refunds, expenses, expense claims,
VAT, journals, accounts, budgets, settlements, receivables, and financial
summaries.

Required anchors:

`payment`, `invoice`, `invoice_line`, `refund`, `expense`,
`vat_rate`, `journal_entry`, `account`, `financial_summary`.

### Asset Management

Cover vehicles, equipment, consumables, stock, suppliers, assignments,
inspections, maintenance schedules, maintenance events, fuel, and insurance.

Required anchors:

`vehicle`, `equipment`, `consumable`, `asset_assignment`,
`asset_inspection`, `maintenance_schedule`, `maintenance_event`,
`fuel_record`, `supplier`.

### Administration & Compliance

Cover contracts, insurance, claims, document storage, retention, policies,
compliance checks, incidents, soft-delete recovery, and edit-version history.

Required anchors:

`contract`, `insurance_policy`, `insurance_claim`, `document`,
`document_version`, `compliance_check`, `data_retention_policy`,
`recovery_request`.

## Platform Modules

### Identity & Access

Support admin, manager, employee, and customer access levels, user accounts,
roles, permissions, assignments, magic links, sessions, authentication
attempts, and RBAC.

Required anchors:

`user_account`, `role`, `permission`, `user_role_assignment`,
`role_permission`, `magic_link`, `user_session`.

### Activity & Audit

Preserve a complete history of changes, edits, and user actions.

Required anchors:

`activity_log`, `audit_log`, `entity_change`, `change_set`.

### Notifications & Automation

Cover notification templates, deliveries, preferences, automation rules,
triggers, actions, and operational/financial hooks.

Required anchors:

`notification`, `notification_template`, `automation_rule`,
`automation_trigger`, `automation_action`.

### API & Integrations

Provide API clients, credentials, endpoints, webhooks, webhook deliveries,
integration mappings, and synchronization runs.

Required anchors:

`api_client`, `api_endpoint`, `webhook`, `webhook_delivery`,
`integration_mapping`.

## KSML Requirements

- Root `name="moving-company-service"`.
- Root `alias_model_name="moving_company_management"`.
- Use current TeaQL repository rules as the source of truth.
- Business attributes use representative literal values.
- Finite sets are modeled as constant objects with sequential IDs beginning at
  1001 and uppercase codes.
- All references must resolve.
- All non-root business objects must be connected to the domain graph.
- Avoid circular-reference depth problems.
- Sensitive fields must use the current audit-mask mechanism.
- No empty attributes, nested business objects, duplicate objects, raw SQL
  concepts, or `_id`-suffixed relationship fields.

## Rust Verification Requirements

- Required `cargo-teaql` version: exactly 2.0.8.
- Generate only `rust-lib-core` followed by `rust-app-console`.
- Never edit `rust-lib-core`.
- Read the generated `rust-app-console/AGENTS.md`.
- Run object-specific assist for every Q, E, create, or update API used.
- Every executing query has `.comment()` and `.purpose()`.
- Every save/update has `.audit_as()`.
- Run `cargo fmt --check`, `cargo check`, `cargo test`, and `cargo run`.
- At least 10 automated tests must pass.
- Exercise at least five distinct Q APIs across different workbenches.
- Exercise at least two E expression chains against runtime-loaded entities.
- Capture SQL logs and assert at least one mutation and one select.

## Required Reports

Write:

- `app-playground/MODEL_REVIEW.md`
- `app-playground/TEAQL_QUICK_TRY_REPORT.md`

The run report must distinguish:

- model pass@1 versus repaired model result;
- Rust pass@1 versus repaired Rust result;
- compile-only checks versus database-backed runtime checks;
- agent-generated code versus human/orchestrator repairs;
- exact tool, model-server, Rust, TeaQL, database, and system configuration.

## Fixed Validation

Run:

```bash
python3 scripts/validate_round4_model.py \
  app-playground/models/model.xml \
  benchmarks/round-4/preserved-core-objects.xml
```

The fixed validator supplements, but does not replace, official TeaQL
evaluation.
