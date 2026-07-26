# Bounded school-service modeling task

Create exactly three business objects in this order:

1. `platform`
   - `name="string()"`
   - `platform_code="string()"`
   - `created_at="createTime()"`
2. `merchant`
   - `name="string()"`
   - `merchant_code="string()"`
   - `contact_email="string()"`
   - `platform="platform()"`
3. `school`
   - `name="string()"`
   - `school_code="string()"`
   - `address="string()"`
   - `merchant="merchant()"`

The root must be `<root name="school-service" data_service="sqlite">`.
Use the listed field names exactly and do not add objects.
