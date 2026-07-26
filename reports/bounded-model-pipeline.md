# Bounded DGX Modeling Pipeline

## Outcome

`scripts/run_bounded_model_pipeline.py` implements a stateless, bounded
generate-validate-repair-finalize workflow for the DGX-hosted
`nemotron-3-super` model.

The pipeline:

1. Reads only an explicit task, grammar example, value whitelist, and optional
   JSON acceptance specification.
2. Rejects prompts that exceed the configured admission budget.
3. Calls the OpenAI-compatible DGX endpoint without tools or thinking.
4. Performs local XML and task-acceptance checks.
5. Calls the official TeaQL evaluator only after local parsing succeeds.
6. Starts a fresh repair request containing only the task package, rejected
   candidate, and bounded validator diagnostics.
7. Stops after the configured repair limit and writes `final-model.xml` only
   after all enabled gates pass.
8. Stops without repair if the TeaQL service does not return an actionable
   error count.

## End-to-end verification

### Fresh generation

- Prompt tokens: 440
- Completion tokens: 138
- DGX duration: 8.908 seconds
- Repair requests: 0
- Local acceptance: 0 errors, 0 warnings
- TeaQL: 0 errors, 10 warnings, 0 suggestions, 10 solids

Evidence:
`artifacts/bounded-model-pipeline-20260727/summary.json`

### Repair of the failed MiMoCode candidate

The seed candidate was
`artifacts/simple-model-test-20260727/raw-model.xml`.

- Initial local errors: invalid root and nested field elements, plus missing
  accepted attributes
- Repair prompt tokens: 898
- Repair completion tokens: 138
- DGX repair duration: 10.253 seconds
- Repair requests: 1
- Final local acceptance: 0 errors, 0 warnings
- Final TeaQL: 0 errors, 10 warnings, 0 suggestions, 10 solids

Evidence:
`artifacts/bounded-model-repair-test-20260727/summary.json`

Both test paths produced byte-identical final models.

## Example

```bash
python3 scripts/run_bounded_model_pipeline.py \
  --base-url http://DGX_HOST:30704 \
  --model nemotron-3-super \
  --task-file benchmarks/pipeline-demo/task.md \
  --grammar-example benchmarks/pipeline-demo/grammar-example.xml \
  --value-whitelist benchmarks/pipeline-demo/value-whitelist.txt \
  --acceptance-spec benchmarks/pipeline-demo/acceptance.json \
  --output-dir artifacts/my-model-run \
  --max-repairs 1 \
  --max-tokens 2048
```
