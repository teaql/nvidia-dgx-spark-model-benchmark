# Official TeaQL Evaluation

- Input: `final-model.xml`
- Endpoint: `https://api.teaql.io/latest/evaluate`
- Exit code: `0`
- Errors: `0`
- Warnings: `141`
- Suggestions: `6`
- Solids: `67`

## Confirmed solids

- The XML document parsed successfully.
- All 30 objects define display name, module, and module-key metadata.
- All 34 object relationships resolve to existing target objects.
- The upload and entrypoint were accepted.

## Warning classification

- 138 `KSML-BUSINESS-002` warnings recommend concrete typical values instead
  of the deliberately restricted `string()` form.
- One `KSML-ROOT-002` warning recommends changing the root name from
  `moving-company-platform` to lowercase kebab-case ending in `-service`.
- One `KSML-DOMAIN-ROOT-002` warning reports six independent roots:
  `address`, `employee`, `service_product`, `marketing_campaign`, `vehicle`,
  and `role_definition`.
- One `KSML-PRIVACY-001-WARN` warning recommends audit masking for
  `customer.billing_address`.

## Suggestions

- Mark `work_log` and `audit_log` with `_log="true"` if they are append-only
  logs.
- Add audit masking for `employee.full_name`, `customer.display_name`,
  `customer_contact.full_name`, and `user_account.username`.

The official evaluator reported no structural or unresolved-reference errors.
