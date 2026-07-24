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
cargo teaql --input models/operations-microservice.xml rust-assist-[action]/[entity-name]
```

> `models/operations-microservice.xml` is the default model path. If the model file is located elsewhere, adjust the `--input` path to match the actual file location in this project.

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
| route_status_type | Route Status Type |
| inventory_condition_type | Inventory Condition Type |
| exception_severity | Exception Severity |
| order_status | Order Status |
| crew_role | Crew Role |
| platform | System Platform |
| merchant | Merchant |
| move_quote | Move Quote |
| move_order | Move Order |
| route_stop | Route Stop |
| crew | Crew |
| crew_member_assignment | Crew Member Assignment |
| vehicle | Vehicle |
| vehicle_assignment | Vehicle Assignment |
| dispatch_assignment | Dispatch Assignment |
| damage_report | Damage Report |
| proof_of_delivery | Proof Of Delivery |
| operational_exception | Operational Exception |
| pickup_instruction | Pickup Instruction |
| delivery_instruction | Delivery Instruction |
| move_inventory | Move Inventory |
| packaging_item | Packaging Item |
| logistics_provider | Logistics Provider |
| third_party_dispatch | Third Party Dispatch |
| fuel_log | Fuel Log |
| maintenance_record | Maintenance Record |
| toll_receipt | Toll Receipt |
| shift_log | Shift Log |
| customer_feedback | Customer Feedback |
| incident_report | Incident Report |


Once the command succeeds, read its output. Use the printed code as a template to write your logic.

If the command cannot be executed, stop and report the missing context. Do not invent APIs.

## Additional References

Read these only when the task requires them:

* **`RUNTIME_CUSTOM_GUIDE.md`**
  Runtime setup, framework APIs (UserContext, SmartList, WebResponse, etc.), and debugging.

* **`TOOL_API_GUIDE.md`**
  Built-in tool integrations (HTTP client, etc.).