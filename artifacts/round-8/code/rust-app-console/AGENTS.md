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
cargo teaql --input models/moving-company-service.xml rust-assist-[action]/[entity-name]
```

> `models/moving-company-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| merchant | Merchant |
| employee | Employee |
| platform_setting | Platform Setting |
| tenant_configuration | Tenant Configuration |
| organization_unit | Organization Unit |
| department_hierarchy | Department Hierarchy |
| branch_office | Branch Office |
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
| proof_of_delivery | Proof of Delivery |
| move_item | Move Item |
| inventory_list | Inventory List |
| packing_material | Packing Material |
| loading_zone | Loading Zone |
| unloading_zone | Unloading Zone |
| transit_log | Transit Log |
| delay_record | Delay Record |
| route_optimization_rule | Route Optimization Rule |
| vehicle_assignment | Vehicle Assignment |
| cargo_weight_record | Cargo Weight Record |
| special_handling_instruction | Special Handling Instruction |
| move_status | Move Status |
| delivery_window | Delivery Window |
| department | Department |
| job_assignment | Job Assignment |
| work_shift | Work Shift |
| worked_hours | Worked Hours |
| payroll_period | Payroll Period |
| payroll_calculation | Payroll Calculation |
| payslip | Payslip |
| bonus | Bonus |
| deduction | Deduction |
| leave_request | Leave Request |
| employee_certification | Employee Certification |
| training_module | Training Module |
| availability_schedule | Availability Schedule |
| skill_profile | Skill Profile |
| performance_review | Performance Review |
| overtime_record | Overtime Record |
| tax_withholding | Tax Withholding |
| benefit_enrollment | Benefit Enrollment |
| shift_swap_request | Shift Swap Request |
| attendance_record | Attendance Record |
| payroll_adjustment | Payroll Adjustment |
| commission_record | Commission Record |
| customer | Customer |
| private_customer_profile | Private Customer Profile |
| corporate_customer_profile | Corporate Customer Profile |
| customer_contact | Customer Contact |
| billing_profile | Billing Profile |
| customer_history | Customer History |
| customer_preference | Customer Preference |
| customer_consent | Customer Consent |
| customer_feedback | Customer Feedback |
| loyalty_tier | Loyalty Tier |
| referral_code | Referral Code |
| communication_log | Communication Log |
| service_rating | Service Rating |
| account_status | Account Status |
| contact_method | Contact Method |
| customer_segment | Customer Segment |
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
| packing_kit | Packing Kit |
| disposal_service | Disposal Service |
| service_area | Service Area |
| availability_calendar | Availability Calendar |
| service_level_agreement | Service Level Agreement |
| add_on_service | Add On Service |
| inventory_item | Inventory Item |
| service_category | Service Category |
| campaign | Campaign |
| discount_code | Discount Code |
| lead | Lead |
| sales_opportunity | Sales Opportunity |
| lead_activity | Lead Activity |
| conversion_event | Conversion Event |
| conversion_metric | Conversion Metric |
| marketing_channel | Marketing Channel |
| audience_segment | Audience Segment |
| promotional_offer | Promotional Offer |
| sales_funnel | Sales Funnel |
| attribution_model | Attribution Model |
| lead_score | Lead Score |
| campaign_budget | Campaign Budget |
| conversion_report | Conversion Report |
| payment | Payment |
| invoice | Invoice |
| invoice_line | Invoice Line |
| refund | Refund |
| expense | Expense |
| vat_rate | VAT Rate |
| journal_entry | Journal Entry |
| account | Account |
| financial_summary | Financial Summary |
| budget | Budget |
| settlement | Settlement |
| receivable | Receivable |
| payable | Payable |
| tax_record | Tax Record |
| currency_rate | Currency Rate |
| payment_method | Payment Method |
| financial_period | Financial Period |
| vehicle | Vehicle |
| equipment | Equipment |
| consumable | Consumable |
| asset_assignment | Asset Assignment |
| asset_inspection | Asset Inspection |
| maintenance_schedule | Maintenance Schedule |
| maintenance_event | Maintenance Event |
| fuel_record | Fuel Record |
| supplier | Supplier |
| inventory_stock | Inventory Stock |
| maintenance_cost | Maintenance Cost |
| vehicle_registration | Vehicle Registration |
| equipment_serial | Equipment Serial |
| supplier_contract | Supplier Contract |
| asset_condition | Asset Condition |
| depreciation_record | Depreciation Record |
| warranty_claim | Warranty Claim |
| storage_location | Storage Location |
| contract | Contract |
| insurance_policy | Insurance Policy |
| insurance_claim | Insurance Claim |
| document | Document |
| document_version | Document Version |
| compliance_check | Compliance Check |
| data_retention_policy | Data Retention Policy |
| recovery_request | Recovery Request |
| policy_document | Policy Document |
| incident_report | Incident Report |
| audit_trail | Audit Trail |
| legal_entity | Legal Entity |
| regulatory_requirement | Regulatory Requirement |
| compliance_certificate | Compliance Certificate |
| user_account | User Account |
| role | Role |
| permission | Permission |
| user_role_assignment | User Role Assignment |
| role_permission | Role Permission |
| magic_link | Magic Link |
| user_session | User Session |
| access_token | Access Token |
| two_factor_auth | Two Factor Auth |
| login_attempt | Login Attempt |
| activity_log | Activity Log |
| audit_log | Audit Log |
| entity_change | Entity Change |
| change_set | Change Set |
| system_event | System Event |
| data_export | Data Export |
| notification | Notification |
| notification_template | Notification Template |
| automation_rule | Automation Rule |
| automation_trigger | Automation Trigger |
| automation_action | Automation Action |
| operational_hook | Operational Hook |
| financial_hook | Financial Hook |
| api_client | API Client |
| api_endpoint | API Endpoint |
| webhook | Webhook |
| webhook_delivery | Webhook Delivery |
| integration_mapping | Integration Mapping |
| synchronization_run | Synchronization Run |
| api_key | API Key |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).