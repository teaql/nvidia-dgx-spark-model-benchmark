# TeaQL Rust Agent Instructions

> [!WARNING]
> **IGNORE GENERIC ORM EXPERIENCE**
>
> Do **not** use pre-trained habits from data-access frameworks, ORMs, or database integration libraries.
>
> Do **not** use SeaORM, Diesel, SQLx, rbatis, or similar frameworks.
>
> Do **not** write raw SQL, DAOs, Repository implementations, or custom persistence layers.
>
> Do **not** guess TeaQL method names.

## How to Write Domain Code

To get the exact API usage and query examples for the entity you are working on, execute the following command:

```bash
cargo teaql --input models/hr-compliance-service.xml rust-assist-[action]/[entity-name]
```

> `models/hr-compliance-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

Replace `[action]` with one of the following:

| action | when-to-use |
|--------|-------------|
| query | Read/find records from the database using Q:: |
| create | Insert a new record into the database |
| update | Modify and save an existing record |
| delete | Remove or soft-delete a record |
| expression | Safely extract nested relation values using E:: |
| list-page | Implement a paginated query returning SmartList |
| debug | View instructions for enabling SQL logging and debugging |

Replace `[entity-name]` with the exact entity-name from the table below:

| entity-name | display-name |
|-------------|--------------|
| customer_profile | Customer Profile |
| customer_contact | Customer Contact |
| customer_address | Customer Address |
| customer_preference | Customer Preference |
| loyalty_program | Loyalty Program |
| customer_feedback | Customer Feedback |
| customer_segment | Customer Segment |
| customer_account | Customer Account |
| payment_method | Payment Method |
| invoice_history | Invoice History |
| dispute_record | Dispute Record |
| service_agreement | Service Agreement |
| contract_terms | Contract Terms |
| renewal_notice | Renewal Notice |
| cancellation_request | Cancellation Request |
| referral_code | Referral Code |
| marketing_campaign | Marketing Campaign |
| lead_source | Lead Source |
| vehicle_registry | Vehicle Registry |
| vehicle_spec | Vehicle Spec |
| maintenance_log | Maintenance Log |
| fuel_record | Fuel Record |
| tire_inventory | Tire Inventory |
| driver_assignment | Driver Assignment |
| driver_license | Driver License |
| driver_training | Driver Training |
| route_plan | Route Plan |
| load_capacity | Load Capacity |
| cargo_securement | Cargo Securement |
| gps_tracking | GPS Tracking |
| telematics_data | Telematics Data |
| incident_report | Incident Report |
| inspection_checklist | Inspection Checklist |
| service_schedule | Service Schedule |
| warranty_info | Warranty Info |
| decommission_record | Decommission Record |
| invoice_template | Invoice Template |
| billing_cycle | Billing Cycle |
| tax_jurisdiction | Tax Jurisdiction |
| tax_rate | Tax Rate |
| discount_policy | Discount Policy |
| payment_gateway | Payment Gateway |
| transaction_log | Transaction Log |
| refund_process | Refund Process |
| credit_note | Credit Note |
| debit_note | Debit Note |
| expense_category | Expense Category |
| cost_center | Cost Center |
| budget_allocation | Budget Allocation |
| financial_report | Financial Report |
| audit_trail | Audit Trail |
| reconciliation_entry | Reconciliation Entry |
| currency_conversion | Currency Conversion |
| fiscal_period | Fiscal Period |
| job_order | Job Order |
| move_schedule | Move Schedule |
| crew_assignment | Crew Assignment |
| equipment_allocation | Equipment Allocation |
| pickup_location | Pickup Location |
| delivery_location | Delivery Location |
| transit_time_estimate | Transit Time Estimate |
| loading_dock | Loading Dock |
| unloading_dock | Unloading Dock |
| customs_documentation | Customs Documentation |
| permit_required | Permit Required |
| insurance_coverage | Insurance Coverage |
| liability_waiver | Liability Waiver |
| tracking_number | Tracking Number |
| status_update | Status Update |
| notification_template | Notification Template |
| sla_metric | SLA Metric |
| performance_kpi | Performance KPI |
| employee_record | Employee Record |
| payroll_info | Payroll Info |
| benefits_plan | Benefits Plan |
| time_off_request | Time Off Request |
| shift_schedule | Shift Schedule |
| performance_review | Performance Review |
| competency_matrix | Competency Matrix |
| training_course | Training Course |
| certification_record | Certification Record |
| safety_incident | Safety Incident |
| hazard_assessment | Hazard Assessment |
| policy_acknowledgment | Policy Acknowledgment |
| grievance_log | Grievance Log |
| disciplinary_action | Disciplinary Action |
| exit_interview | Exit Interview |
| onboarding_checklist | Onboarding Checklist |
| offboarding_checklist | Offboarding Checklist |
| compliance_audit | Compliance Audit |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).