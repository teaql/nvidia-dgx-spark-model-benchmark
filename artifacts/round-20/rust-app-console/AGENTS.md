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
cargo teaql --input models/service-scheduling-service.xml rust-assist-[action]/[entity-name]
```

> `models/service-scheduling-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| address_book | Address Book |
| contact_person | Contact Person |
| account_settings | Account Settings |
| loyalty_program | Loyalty Program |
| service_history | Service History |
| feedback_review | Feedback Review |
| dispute_case | Dispute Case |
| document_upload | Document Upload |
| preference_center | Preference Center |
| notification_pref | Notification Preference |
| billing_contact | Billing Contact |
| vehicle_registry | Vehicle Registry |
| driver_profile | Driver Profile |
| maintenance_log | Maintenance Log |
| fuel_record | Fuel Record |
| inspection_checklist | Inspection Checklist |
| route_plan | Route Plan |
| load_manifest | Load Manifest |
| equipment_inventory | Equipment Inventory |
| garage_assignment | Garage Assignment |
| incident_report | Incident Report |
| compliance_certificate | Compliance Certificate |
| telematics_data | Telematics Data |
| invoice | Invoice |
| payment_transaction | Payment Transaction |
| tax_calculation | Tax Calculation |
| credit_memo | Credit Memo |
| deposit_receipt | Deposit Receipt |
| refund_request | Refund Request |
| expense_report | Expense Report |
| budget_allocation | Budget Allocation |
| financial_statement | Financial Statement |
| audit_trail | Audit Trail |
| currency_exchange | Currency Exchange |
| receivable_aging | Receivable Aging |
| employee_record | Employee Record |
| payroll_run | Payroll Run |
| timesheet_entry | Timesheet Entry |
| benefit_plan | Benefit Plan |
| tax_withholding | Tax Withholding |
| leave_request | Leave Request |
| training_record | Training Record |
| performance_review | Performance Review |
| compensation_adjustment | Compensation Adjustment |
| onboarding_checklist | Onboarding Checklist |
| offboarding_process | Offboarding Process |
| employee_handbook | Employee Handbook |
| move_order | Move Order |
| job_schedule | Job Schedule |
| crew_assignment | Crew Assignment |
| equipment_allocation | Equipment Allocation |
| time_slot | Time Slot |
| service_location | Service Location |
| special_instructions | Special Instructions |
| status_update | Status Update |
| cancellation_policy | Cancellation Policy |
| reschedule_request | Reschedule Request |
| satisfaction_survey | Satisfaction Survey |
| follow_up_task | Follow Up Task |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).