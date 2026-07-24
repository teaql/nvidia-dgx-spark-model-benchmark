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
| platform | System Platform |
| merchant | Merchant |
| employee | Employee |
| platform_setting | Platform Setting |
| platform_user | Platform User |
| platform_audit_log | Platform Audit Log |
| organization | Organization |
| organization_setting | Organization Setting |
| organization_member | Organization Member |
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
| inventory_item | Inventory Item |
| packing_list | Packing List |
| packing_item | Packing Item |
| loading_plan | Loading Plan |
| unloading_plan | Unloading Plan |
| storage_facility | Storage Facility |
| storage_unit | Storage Unit |
| storage_inventory | Storage Inventory |
| transport_manifest | Transport Manifest |
| customs_declaration | Customs Declaration |
| equipment_checklist | Equipment Checklist |
| fuel_log | Fuel Log |
| maintenance_request | Maintenance Request |
| department | Department |
| job_assignment | Job Assignment |
| work_shift | Work Shift |
| worked_hours | Worked Hours |
| payroll_period | Payroll Period |
| payroll_calculation | Payroll Calculation |
| payslip | Payslip |
| bonus | Bonus |
| employee_certification | Employee Certification |
| leave_request | Leave Request |
| billing_profile | Billing Profile |
| corporate_customer_profile | Corporate Customer Profile |
| customer | Customer |
| customer_consent | Customer Consent |
| customer_contact | Customer Contact |
| customer_history | Customer History |
| customer_preference | Customer Preference |
| private_customer_profile | Private Customer Profile |
| box_rental | Box Rental |
| cleaning_service | Cleaning Service |
| moving_service | Moving Service |
| price_list | Price List |
| product | Product |
| service | Service |
| service_bundle | Service Bundle |
| service_configuration | Service Configuration |
| service_price | Service Price |
| campaign | Campaign |
| conversion_event | Conversion Event |
| conversion_metric | Conversion Metric |
| discount_code | Discount Code |
| lead | Lead |
| lead_activity | Lead Activity |
| sales_opportunity | Sales Opportunity |
| account | Account |
| expense | Expense |
| financial_summary | Financial Summary |
| invoice | Invoice |
| invoice_line | Invoice Line |
| journal_entry | Journal Entry |
| payment | Payment |
| refund | Refund |
| vat_rate | Vat Rate |
| asset_assignment | Asset Assignment |
| asset_inspection | Asset Inspection |
| consumable | Consumable |
| equipment | Equipment |
| fuel_record | Fuel Record |
| maintenance_event | Maintenance Event |
| maintenance_schedule | Maintenance Schedule |
| supplier | Supplier |
| vehicle | Vehicle |
| compliance_check | Compliance Check |
| contract | Contract |
| data_retention_policy | Data Retention Policy |
| document | Document |
| document_version | Document Version |
| insurance_claim | Insurance Claim |
| insurance_policy | Insurance Policy |
| recovery_request | Recovery Request |
| magic_link | Magic Link |
| permission | Permission |
| role | Role |
| role_permission | Role Permission |
| user_account | User Account |
| user_role_assignment | User Role Assignment |
| user_session | User Session |
| activity_log | Activity Log |
| audit_log | Audit Log |
| change_set | Change Set |
| entity_change | Entity Change |
| automation_action | Automation Action |
| automation_rule | Automation Rule |
| automation_trigger | Automation Trigger |
| notification | Notification |
| notification_template | Notification Template |
| api_client | Api Client |
| api_endpoint | Api Endpoint |
| integration_mapping | Integration Mapping |
| webhook | Webhook |
| webhook_delivery | Webhook Delivery |
| platform_configuration | Platform Configuration |
| platform_locale | Platform Locale |
| merchant_branch | Merchant Branch |
| merchant_setting | Merchant Setting |
| operational_exception | Operational Exception |
| crew_member_assignment | Crew Member Assignment |
| pickup_instruction | Pickup Instruction |
| delivery_instruction | Delivery Instruction |
| move_inventory | Move Inventory |
| extra_operations_logistics_1 | Extra Operations Logistics 1 |
| extra_operations_logistics_2 | Extra Operations Logistics 2 |
| extra_operations_logistics_3 | Extra Operations Logistics 3 |
| extra_operations_logistics_4 | Extra Operations Logistics 4 |
| extra_operations_logistics_5 | Extra Operations Logistics 5 |
| extra_operations_logistics_6 | Extra Operations Logistics 6 |
| extra_operations_logistics_7 | Extra Operations Logistics 7 |
| extra_operations_logistics_8 | Extra Operations Logistics 8 |
| extra_operations_logistics_9 | Extra Operations Logistics 9 |
| employee_availability | Employee Availability |
| payroll_deduction | Payroll Deduction |
| training_session | Training Session |
| shift_assignment | Shift Assignment |
| extra_employees_payroll_1 | Extra Employees Payroll 1 |
| extra_employees_payroll_2 | Extra Employees Payroll 2 |
| extra_employees_payroll_3 | Extra Employees Payroll 3 |
| extra_employees_payroll_4 | Extra Employees Payroll 4 |
| extra_employees_payroll_5 | Extra Employees Payroll 5 |
| extra_employees_payroll_6 | Extra Employees Payroll 6 |
| extra_employees_payroll_7 | Extra Employees Payroll 7 |
| customer_complaint | Customer Complaint |
| customer_note | Customer Note |
| extra_customer_management_1 | Extra Customer Management 1 |
| extra_customer_management_2 | Extra Customer Management 2 |
| extra_customer_management_3 | Extra Customer Management 3 |
| extra_customer_management_4 | Extra Customer Management 4 |
| extra_customer_management_5 | Extra Customer Management 5 |
| extra_customer_management_6 | Extra Customer Management 6 |
| storage_service | Storage Service |
| packing_service | Packing Service |
| disposal_service | Disposal Service |
| rental_period | Rental Period |
| service_area | Service Area |
| extra_products_services_1 | Extra Products Services 1 |
| extra_products_services_2 | Extra Products Services 2 |
| extra_products_services_3 | Extra Products Services 3 |
| extra_products_services_4 | Extra Products Services 4 |
| campaign_audience | Campaign Audience |
| campaign_channel | Campaign Channel |
| lead_attribution | Lead Attribution |
| sales_funnel | Sales Funnel |
| extra_marketing_sales_1 | Extra Marketing Sales 1 |
| extra_marketing_sales_2 | Extra Marketing Sales 2 |
| extra_marketing_sales_3 | Extra Marketing Sales 3 |
| extra_marketing_sales_4 | Extra Marketing Sales 4 |
| expense_claim | Expense Claim |
| settlement | Settlement |
| receivable | Receivable |
| payable | Payable |
| extra_finance_accounting_1 | Extra Finance Accounting 1 |
| extra_finance_accounting_2 | Extra Finance Accounting 2 |
| extra_finance_accounting_3 | Extra Finance Accounting 3 |
| extra_finance_accounting_4 | Extra Finance Accounting 4 |
| vehicle_inspection | Vehicle Inspection |
| equipment_checkout | Equipment Checkout |
| consumable_reorder | Consumable Reorder |
| extra_asset_management_1 | Extra Asset Management 1 |
| extra_asset_management_2 | Extra Asset Management 2 |
| extra_asset_management_3 | Extra Asset Management 3 |
| extra_asset_management_4 | Extra Asset Management 4 |
| extra_asset_management_5 | Extra Asset Management 5 |
| authentication_attempt | Authentication Attempt |
| access_policy | Access Policy |
| extra_identity_access_1 | Extra Identity Access 1 |
| audit_export | Audit Export |
| extra_activity_audit_1 | Extra Activity Audit 1 |
| notification_preference | Notification Preference |
| notification_delivery | Notification Delivery |
| synchronization_run | Synchronization Run |
| extra_api_integrations_1 | Extra Api Integrations 1 |
| gender_type | Gender Type |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).