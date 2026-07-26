# Moving-company platform: 30-object bounded model

Create exactly the following 30 business objects in the listed order. Use
exactly the listed business attributes and relationship value forms.

Every object must also contain:

- `record_version="string()"`
- `deleted_at="string()"`
- `updated_at="updateTime()"`

## Operations and logistics

1. `address`: `label`, `line1`, `city`, `postal_code`
2. `move_order`: `order_number`, `customer="customer()"`,
   `pickup_address="address()"`, `delivery_address="address()"`,
   `contract_document_url`
3. `route_plan`: `move_order="move_order()"`, `route_code`, `planned_distance`
4. `time_slot`: `move_order="move_order()"`, `slot_start`, `slot_end`
5. `fulfillment_event`: `move_order="move_order()"`, `event_code`,
    `event_status`, `automation_hook`

## Employees and payroll

6. `employee`: `employee_code`, `full_name`, `payroll_number`
7. `job_assignment`: `employee="employee()"`, `move_order="move_order()"`,
    `assignment_role`
8. `work_log`: `job_assignment="job_assignment()"`, `worked_date`,
    `worked_hours`
9. `payroll_record`: `employee="employee()"`, `pay_period`, `gross_amount`,
    `bonus_amount`
10. `leave_request`: `employee="employee()"`, `leave_category`, `start_date`,
    `end_date`, `approval_status`

## Customer management

11. `customer`: `customer_code`, `display_name`,
    `billing_address="address()"`
12. `private_customer_profile`: `customer="customer()"`, `legal_name`,
    `personal_identifier`, `_audit_mask_fields="personal_identifier"`
13. `corporate_customer_profile`: `customer="customer()"`, `company_name`,
    `vat_number`, `_audit_mask_fields="vat_number"`
14. `customer_contact`: `customer="customer()"`, `full_name`, `email`, `phone`,
    `_audit_mask_fields="email,phone"`

## Products and services

15. `service_product`: `service_code`, `service_name`, `service_category`,
    `base_price`, `configuration_schema`
16. `moving_service`: `service_product="service_product()"`, `crew_size`,
    `duration_band`
17. `cleaning_service`: `service_product="service_product()"`,
    `cleaning_scope`, `duration_band`
18. `box_rental`: `service_product="service_product()"`, `box_size`,
    `rental_period`

## Marketing and sales

19. `marketing_campaign`: `campaign_code`, `campaign_name`, `channel`
20. `discount_code`: `marketing_campaign="marketing_campaign()"`,
    `service_product="service_product()"`, `code`, `discount_value`
21. `sales_lead`: `customer="customer()"`,
    `marketing_campaign="marketing_campaign()"`, `lead_stage`,
    `conversion_metric`

## Finance and accounting

22. `invoice`: `move_order="move_order()"`, `customer="customer()"`,
    `invoice_number`, `subtotal`, `vat_amount`, `total_amount`
23. `payment`: `invoice="invoice()"`, `payment_reference`, `payment_method`,
    `paid_amount`
24. `expense`: `move_order="move_order()"`, `employee="employee()"`,
    `expense_category`, `expense_amount`

## Asset management

25. `vehicle`: `vehicle_code`, `registration_number`, `insurance_document_url`
26. `equipment`: `equipment_code`, `equipment_name`, `vehicle="vehicle()"`
27. `maintenance_schedule`: `vehicle="vehicle()"`, `equipment="equipment()"`,
    `scheduled_date`, `maintenance_status`

## Platform, access, and audit

28. `role_definition`: `role_code`, `role_name`, `permission_set`
29. `user_account`: `username`, `email`,
    `role_definition="role_definition()"`, `customer="customer()"`,
    `employee="employee()"`, `magic_link_token`,
    `_audit_mask_fields="email,magic_link_token"`
30. `audit_log`: `user_account="user_account()"`, `action_name`, `object_name`,
    `object_identifier`, `changed_at="createTime()"`

Use `string()` for every listed attribute that has no explicit value form.
The root must be:

`<root name="moving-company-platform" data_service="sqlite">`

Assign coherent `_name`, `_module`, and `_module_key` metadata according to the
section headings. Do not add objects or business attributes.
