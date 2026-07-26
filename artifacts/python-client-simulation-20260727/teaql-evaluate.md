# TeaQL Evaluation Result

- Command: `cargo teaql evaluate --input artifacts/python-client-simulation-20260727/model.xml`
- Endpoint: `https://api.teaql.io/latest/evaluate`
- Exit code: `0`
- Errors: `0`
- Warnings: `10`
- Suggestions: `0`
- Solids: `10`

## Confirmed solids

- The XML document parsed successfully.
- The `school-service` root name is well-formed.
- All three objects define display name, module, and module-key metadata.
- `merchant.platform` resolves to `platform`.
- `school.merchant` resolves to `merchant`.
- Exactly one domain-root candidate, `platform`, was found.

## Warning classification

- Eight `KSML-BUSINESS-002` warnings recommend rich concrete example values
  instead of generic `string()` values.
- Two `KSML-MODULE-003` warnings report that the `Merchants` and `Schools`
  modules contain only one object each.

The evaluator reported no structural or reference errors.
