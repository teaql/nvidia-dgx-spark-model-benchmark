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
cargo teaql --input models/hr-payroll-microservice.xml rust-assist-[action]/[entity-name]
```

> `models/hr-payroll-microservice.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| leave_type | Leave Type |
| employee_status | Employee Status |
| contract_type | Contract Type |
| review_grade | Review Grade |
| application_status | Application Status |
| platform | System Platform |
| merchant | Merchant |
| department | Department |
| position | Position |
| employee | Employee |
| salary_record | Salary Record |
| attendance_log | Attendance Log |
| leave_request | Leave Request |
| performance_review | Performance Review |
| training_record | Training Record |
| benefit_plan | Benefit Plan |
| expense_claim | Expense Claim |
| payroll_run | Payroll Run |
| tax_form | Tax Form |
| contract | Contract |
| resignation | Resignation |
| warning_letter | Warning Letter |
| bonus_record | Bonus Record |
| shift_schedule | Shift Schedule |
| time_off_balance | Time Off Balance |
| recruitment_post | Recruitment Post |
| job_application | Job Application |
| interview | Interview |
| offer_letter | Offer Letter |
| onboarding_checklist | Onboarding Checklist |
| offboarding_checklist | Offboarding Checklist |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).