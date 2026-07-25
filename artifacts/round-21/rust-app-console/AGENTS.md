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
cargo teaql --input models/human-resources-service.xml rust-assist-[action]/[entity-name]
```

> `models/human-resources-service.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| customer | Customer |
| lead | Lead |
| quote | Quote |
| contract | Contract |
| invoice | Invoice |
| payment | Payment |
| sales_order | Sales Order |
| sales_rep | Sales Rep |
| territory | Territory |
| pricing | Pricing |
| discount | Discount |
| promotion | Promotion |
| campaign | Campaign |
| feedback | Feedback |
| complaint | Complaint |
| service_request | Service Request |
| warranty | Warranty |
| renewal | Renewal |
| upsell | Upsell |
| cross_sell | Cross Sell |
| truck | Truck |
| trailer | Trailer |
| driver | Driver |
| move_order | Move Order |
| route | Route |
| schedule | Schedule |
| load | Load |
| unload | Unload |
| warehouse | Warehouse |
| inventory | Inventory |
| pallet | Pallet |
| shipment | Shipment |
| tracking | Tracking |
| fuel_log | Fuel Log |
| maintenance | Maintenance |
| inspection | Inspection |
| safety_report | Safety Report |
| crew | Crew |
| equipment | Equipment |
| facility | Facility |
| budget | Budget |
| expense | Expense |
| revenue | Revenue |
| profit | Profit |
| loss | Loss |
| tax | Tax |
| audit | Audit |
| ledger | Ledger |
| journal | Journal |
| accounts_payable | Accounts Payable |
| accounts_receivable | Accounts Receivable |
| payroll | Payroll |
| expense_report | Expense Report |
| budget_forecast | Budget Forecast |
| cash_flow | Cash Flow |
| investment | Investment |
| asset | Asset |
| liability | Liability |
| equity | Equity |
| financial_statement | Financial Statement |
| employee | Employee |
| contractor | Contractor |
| beneficiary | Beneficiary |
| dependent | Dependent |
| training | Training |
| certification | Certification |
| performance_review | Performance Review |
| termination | Termination |
| onboarding | Onboarding |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).