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
cargo teaql --input models/reporting-service.xml rust-assist-[action]/[entity-name]
```

> `models/reporting-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| customer_contract | Customer Contract |
| customer_feedback | Customer Feedback |
| customer_segment | Customer Segment |
| customer_loyalty | Customer Loyalty |
| customer_invoice | Customer Invoice |
| customer_payment | Customer Payment |
| customer_claim | Customer Claim |
| customer_notification | Customer Notification |
| customer_account | Customer Account |
| customer_lead | Customer Lead |
| customer_quote | Customer Quote |
| customer_service | Customer Service |
| customer_support_ticket | Customer Support Ticket |
| customer_vehicle | Customer Vehicle |
| customer_move_history | Customer Move History |
| customer_preferred_time | Customer Preferred Time |
| fleet_vehicle | Fleet Vehicle |
| vehicle_spec | Vehicle Spec |
| vehicle_maintenance | Vehicle Maintenance |
| vehicle_inspection | Vehicle Inspection |
| vehicle_assignment | Vehicle Assignment |
| vehicle_utilization | Vehicle Utilization |
| vehicle_fuel_log | Vehicle Fuel Log |
| vehicle_odometer | Vehicle Odometer |
| vehicle_insurance | Vehicle Insurance |
| vehicle_registration | Vehicle Registration |
| vehicle_damage_report | Vehicle Damage Report |
| vehicle_cleaning_schedule | Vehicle Cleaning Schedule |
| fleet_operator | Fleet Operator |
| driver_profile | Driver Profile |
| driver_license | Driver License |
| driver_certification | Driver Certification |
| driver_availability | Driver Availability |
| driver_performance | Driver Performance |
| driver_training | Driver Training |
| fleet_dispatch | Fleet Dispatch |
| invoice_header | Invoice Header |
| invoice_line_item | Invoice Line Item |
| payment_method | Payment Method |
| payment_transaction | Payment Transaction |
| billing_cycle | Billing Cycle |
| tax_code | Tax Code |
| discount_rule | Discount Rule |
| credit_note | Credit Note |
| debit_note | Debit Note |
| billing_address | Billing Address |
| outstanding_balance | Outstanding Balance |
| aging_report | Aging Report |
| payment_reminder | Payment Reminder |
| refund_request | Refund Request |
| billing_adjustment | Billing Adjustment |
| revenue_recognition | Revenue Recognition |
| financial_period | Financial Period |
| audit_trail | Audit Trail |
| currency_rate | Currency Rate |
| billing_approval | Billing Approval |
| move_order | Move Order |
| move_schedule | Move Schedule |
| route_plan | Route Plan |
| load_plan | Load Plan |
| crew_assignment | Crew Assignment |
| equipment_checklist | Equipment Checklist |
| loading_procedure | Loading Procedure |
| unloading_procedure | Unloading Procedure |
| transit_monitoring | Transit Monitoring |
| delivery_confirmation | Delivery Confirmation |
| exception_handling | Exception Handling |
| customs_documentation | Customs Documentation |
| inventory_snapshot | Inventory Snapshot |
| warehouse_allocation | Warehouse Allocation |
| dock_scheduling | Dock Scheduling |
| yard_management | Yard Management |
| safety_incident | Safety Incident |
| compliance_check | Compliance Check |
| performance_metric | Performance Metric |
| operations_dashboard | Operations Dashboard |
| daily_summary | Daily Summary |
| weekly_report | Weekly Report |
| monthly_kpi | Monthly KPI |
| annual_performance | Annual Performance |
| utilization_report | Utilization Report |
| cost_analysis | Cost Analysis |
| profit_margin | Profit Margin |
| customer_satisfaction | Customer Satisfaction |
| on_time_delivery | On Time Delivery |
| claim_rate | Claim Rate |
| fleet_efficiency | Fleet Efficiency |
| driver_productivity | Driver Productivity |
| billing_accuracy | Billing Accuracy |
| invoice_aging | Invoice Aging |
| move_volume_trend | Move Volume Trend |
| geographic_distribution | Geographic Distribution |
| service_line_performance | Service Line Performance |
| expense_variance | Expense Variance |
| forecast_vs_actual | Forecast vs Actual |
| executive_dashboard | Executive Dashboard |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).