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
cargo teaql --input models/module-7-service.xml rust-assist-[action]/[entity-name]
```

> `models/module-7-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| platform | Platform |
| platform_config | Platform Config |
| tenant_registry | Tenant Registry |
| merchant | Merchant |
| branch | Branch |
| franchise | Franchise |
| move_order | Move Order |
| move_quote | Move Quote |
| route | Route |
| route_stop | Route Stop |
| time_slot | Time Slot |
| fulfillment_event | Fulfillment Event |
| address | Address |
| crew | Crew |
| dispatch_assignment | Dispatch Assignment |
| damage_report | Damage Report |
| proof_of_delivery | Proof Of Delivery |
| packing_list | Packing List |
| inventory_item | Inventory Item |
| vehicle_load_plan | Vehicle Load Plan |
| weigh_station_ticket | Weigh Station Ticket |
| toll_receipt | Toll Receipt |
| parking_permit | Parking Permit |
| traffic_violation | Traffic Violation |
| detour_log | Detour Log |
| fuel_stop | Fuel Stop |
| weather_delay | Weather Delay |
| customer_signature | Customer Signature |
| walkthrough_checklist | Walkthrough Checklist |
| post_move_survey | Post Move Survey |
| operations_manager_override | Operations Manager Override |
| employee | Employee |
| department | Department |
| job_assignment | Job Assignment |
| work_shift | Work Shift |
| worked_hours | Worked Hours |
| payroll_period | Payroll Period |
| payroll_calculation | Payroll Calculation |
| payslip | Payslip |
| bonus | Bonus |
| leave_request | Leave Request |
| employee_certification | Employee Certification |
| tax_withholding | Tax Withholding |
| direct_deposit_info | Direct Deposit Info |
| union_dues | Union Dues |
| overtime_approval | Overtime Approval |
| expense_reimbursement | Expense Reimbursement |
| performance_review | Performance Review |
| warning_letter | Warning Letter |
| termination_record | Termination Record |
| emergency_contact | Emergency Contact |
| uniform_assignment | Uniform Assignment |
| background_check | Background Check |
| customer | Customer |
| private_customer_profile | Private Customer Profile |
| corporate_customer_profile | Corporate Customer Profile |
| customer_contact | Customer Contact |
| billing_profile | Billing Profile |
| customer_history | Customer History |
| customer_preference | Customer Preference |
| customer_consent | Customer Consent |
| referral_code | Referral Code |
| loyalty_tier | Loyalty Tier |
| complaint_ticket | Complaint Ticket |
| resolution_offer | Resolution Offer |
| vip_status | VIP Status |
| do_not_contact_list | Do Not Contact List |
| customer_note | Customer Note |
| communication_log | Communication Log |
| product | Product |
| service | Service |
| moving_service | Moving Service |
| cleaning_service | Cleaning Service |
| box_rental | Box Rental |
| service_configuration | Service Configuration |
| price_list | Price List |
| service_price | Service Price |
| service_bundle | Service Bundle |
| storage_unit | Storage Unit |
| packing_material | Packing Material |
| insurance_addon | Insurance Addon |
| piano_handling | Piano Handling |
| stair_fee | Stair Fee |
| long_carry_fee | Long Carry Fee |
| hoisting_service | Hoisting Service |
| vehicle_transport | Vehicle Transport |
| pet_relocation_service | Pet Relocation Service |
| campaign | Campaign |
| discount_code | Discount Code |
| lead | Lead |
| sales_opportunity | Sales Opportunity |
| lead_activity | Lead Activity |
| conversion_event | Conversion Event |
| conversion_metric | Conversion Metric |
| ad_spend | Ad Spend |
| social_media_post | Social Media Post |
| email_blast | Email Blast |
| sms_campaign | SMS Campaign |
| sales_script | Sales Script |
| objection_handling_guide | Objection Handling Guide |
| competitor_analysis | Competitor Analysis |
| sales_territory | Sales Territory |
| payment | Payment |
| invoice | Invoice |
| invoice_line | Invoice Line |
| refund | Refund |
| expense | Expense |
| vat_rate | VAT Rate |
| journal_entry | Journal Entry |
| account | Account |
| financial_summary | Financial Summary |
| tax_document | Tax Document |
| bank_transaction | Bank Transaction |
| merchant_fee | Merchant Fee |
| chargeback_record | Chargeback Record |
| credit_note | Credit Note |
| debit_note | Debit Note |
| audit_adjustment | Audit Adjustment |
| fiscal_year | Fiscal Year |
| vehicle | Vehicle |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).