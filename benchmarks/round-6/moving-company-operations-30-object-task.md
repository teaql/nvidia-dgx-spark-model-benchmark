# Round 6 Task — Moving Company Operations Microservice

## Objective

Build a targeted, microservice-sized moving-company operations platform using a semantic TeaQL KSML model and Rust runtime. This round intentionally restricts the domain to ~30 core business objects to mimic real-world Bounded Contexts (DDD).

Work model-first:

1. Create the semantic KSML model.
2. Run `cargo teaql --input <model> evaluate`.
3. Repair until evaluation has zero errors.
4. Review the model and record `confirmed_with_assumptions`.
5. Generate `rust-lib-core`, then `rust-app-console`.
6. Compile, test Q and E APIs against a real local database, and write a run report.

## Required Architecture

You are building the **Operations & Logistics** Bounded Context. 
The minimum required size for this model is **30 direct child objects**. The total object count MUST be exactly 30 or slightly above.

### Core Required Entities
You must include the following core entities in the `Operations & Logistics` module (`_module_key="operations-logistics"`):
- `move_order`
- `move_quote`
- `route_stop`
- `crew`
- `dispatch_assignment`
- `damage_report`
- `proof_of_delivery`
- `operational_exception`
- `crew_member_assignment`
- `pickup_instruction`
- `delivery_instruction`
- `move_inventory`

### Supplementary Entities
Generate realistic supplementary and constant entities to reach the ~30 object requirement. Examples include: `packaging_item`, `vehicle`, `vehicle_assignment`, `route_status_type`, `inventory_condition_type`, etc.

### Preserved Core Dependencies
Even though this is a microservice, it still connects to the multi-tenant SaaS architecture. You must include these foundational objects exactly as they are defined below:

```xml
  <platform
      _name="System Platform"
      _module="Platform Administration"
      _module_key="platform-administration"
      name="Moving Company Platform"
      create_time="createTime()"
      update_time="updateTime()"/>

  <merchant
      _name="Merchant"
      _module="Organization Administration"
      _module_key="organization-administration"
      name="Nordic Moving Services"
      tax_number="FI12345678"
      address="Mannerheimintie 10, Helsinki"
      external_id="MERCHANT_NORDIC_001"
      platform="platform()"
      create_time="createTime()"
      update_time="updateTime()"/>
```

## Model Requirements

1. **Size**: At least 30 objects total.
2. **Naming**: No trailing numbers for padding (e.g., `entity_1`, `object_2`). Use distinct, realistic domain terms.
3. **Syntax**: Adhere strictly to the standard TeaQL KSML format. Every object MUST be an XML tag named after its domain model (e.g., `<move_order _name="Move Order" ... />`). Do NOT wrap in `<object name="...">`.
4. **Fields**: Every business object must include concrete, realistic business fields plus `merchant="merchant(context)"`, `create_time="createTime()"`, and `update_time="updateTime()"`. Constant objects do not need `merchant` but require `platform="platform()"`.

## Outputs
- Save your generated model to `artifacts/round-6/model.xml`.
