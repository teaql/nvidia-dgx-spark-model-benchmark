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
cargo teaql --input models/operations-management-service.xml rust-assist-[action]/[entity-name]
```

> `models/operations-management-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| invoice | Invoice |
| bill | Bill |
| payment | Payment |
| expense | Expense |
| revenue | Revenue |
| ledger | Ledger |
| audit | Audit |
| tax | Tax |
| budget | Budget |
| forecast | Forecast |
| payroll | Payroll |
| expense_report | Expense Report |
| credit | Credit |
| debit | Debit |
| balance | Balance |
| asset | Asset |
| liability | Liability |
| equity | Equity |
| cash_flow | Cash Flow |
| financial_statement | Financial Statement |
| shipment | Shipment |
| route | Route |
| vehicle | Vehicle |
| driver | Driver |
| load | Load |
| unload | Unload |
| capacity | Capacity |
| manifest | Manifest |
| tracking | Tracking |
| dispatch | Dispatch |
| freight | Freight |
| carrier | Carrier |
| warehouse | Warehouse |
| loading_dock | Loading Dock |
| unloading_dock | Unloading Dock |
| freight_forwarder | Freight Forwarder |
| customs | Customs |
| documentation | Documentation |
| toll | Toll |
| fuel | Fuel |
| customer | Customer |
| client | Client |
| contact | Contact |
| lead | Lead |
| prospect | Prospect |
| account | Account |
| service_agreement | Service Agreement |
| contract | Contract |
| warranty | Warranty |
| support_ticket | Support Ticket |
| feedback | Feedback |
| survey | Survey |
| loyalty_program | Loyalty Program |
| referral | Referral |
| discount | Discount |
| promotion | Promotion |
| marketing_campaign | Marketing Campaign |
| newsletter | Newsletter |
| communication_preference | Communication Preference |
| profile | Profile |
| maintenance | Maintenance |
| repair | Repair |
| inspection | Inspection |
| safety_check | Safety Check |
| incident_report | Incident Report |
| claim | Claim |
| parts_inventory | Parts Inventory |
| stock_level | Stock Level |
| reorder_point | Reorder Point |
| supplier | Supplier |
| vendor | Vendor |
| purchase_order | Purchase Order |
| receiving | Receiving |
| putaway | Putaway |
| picking | Picking |
| packing | Packing |
| shipping | Shipping |
| returns_process | Returns Process |
| quality_control | Quality Control |
| performance_metric | Performance Metric |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).