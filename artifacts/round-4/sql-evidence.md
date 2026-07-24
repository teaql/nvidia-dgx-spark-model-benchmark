# Round 4 SQL evidence

The final isolated SQLite run produced 74 `SELECT` statements and 36,900
`INSERT` statements. It produced no update or delete statements. Counts were
taken from the second successful execution in TeaQL's appended runtime log.

Representative application queries:

```sql
SELECT id, version, record_name, merchant AS merchant_id
FROM move_order_data
WHERE (version > 0)
LIMIT 20;

SELECT id, version, name, customer_type AS customer_type_id,
       status AS status_id, merchant AS merchant_id
FROM customer_data
WHERE (version > 0)
LIMIT 20;

SELECT id, version, employee_number, name, job_title,
       merchant AS merchant_id
FROM employee_data
WHERE (version > 0)
LIMIT 20;

SELECT id, version, invoice_number, total_amount, vat_amount,
       status AS status_id, merchant AS merchant_id
FROM invoice_data
WHERE (version > 0)
LIMIT 20;

SELECT id, version, license_plate, mileage_km, last_service_date,
       vehicle_type AS vehicle_type_id, merchant AS merchant_id
FROM vehicle_data
WHERE (version > 0)
LIMIT 20;
```

Representative generated mutation:

```sql
INSERT INTO worked_hour_entry_data
  (id, work_date, start_time, end_time, break_minutes, total_hours,
   is_overtime, create_time, version, employee, task_performed, merchant)
VALUES
  (300, '2025-09-28', '00:00 300', '30:00 300', 30, 300,
   FALSE, '2025-09-28T00:00:00+00:00', 1, 100, 300, 1);
```

The full raw log was 53 MB because sample generation logged every inserted
entity. This compact evidence file preserves the counts and representative
statements without adding that transient log to the repository.
