# Round 12: Microservice Benchmark (30 Objects) on Qwen 35B

## Overview
This round was initiated to establish a control group. While previous rounds tested 30-object generations successfully, the user noted that we had never tested the **30-object scenario on the current `qwen3.6-35b-a3b` model** (64K context on DGX). This test determines if the syntax hallucinations (e.g., `<module>`, `<field>`) are caused by the context length/object count, or if they are a fundamental limitation of the 35B model itself.

## Execution
- **Context Length**: 64K
- **Model Used**: `/home/dgx007/models/qwen3.6-35b-a3b`
- **Target Size**: Exactly 30 Objects (2 Modules).
- **Generation Time**: 2.5 minutes (157 seconds).

## Observations
1. **No Catastrophic Degeneration**: Unlike the 60-object test (Round 11) which fell into an infinite vocabulary loop, and the 180-object test (Round 8/9) which created impossible cyclic dependencies, the model **successfully completed the logic generation** for 30 objects!
2. **Persistent Syntax Hallucinations (202 NATIVE ERRORS)**: Despite generating the logic flawlessly, the model **still hallucinated the KSML syntax**. It generated hierarchical `<module>` folders and `<field>` tags instead of flat objects and `<attribute>` tags.
3. **AST Rescue**: Because the internal logical relationships and sizes were manageable, the AST parser script (`transform_to_ksml_v4.py`) easily flattened and converted the hallucinated XML into standard KSML. 
4. **Final Result**: After the automated script correction and a minor privacy tag fix, the `cargo teaql evaluate` engine returned **0 Errors (34 Solid Objects)**.

## Conclusion
Testing the 30-object threshold on `Qwen-3.6-35B` has yielded a critical insight into separating **Logical Capacity** from **Syntax Compliance**:
- **Syntax Compliance is tied to Model Capability (Size)**: The 35B model simply struggles to follow rigid XML schema instructions (like "don't use hierarchical modules", "use `<attribute>`"). It will always hallucinate the syntax, whether generating 30 objects or 180 objects.
- **Logical Capacity is tied to Object Count**: The model's "Working Memory" can map out the relationships of ~30 objects without creating cyclic recursion or falling into infinite loops. Beyond 30 objects (e.g., 60+), the attention mechanism collapses, leading to catastrophic failure.

**Final Verdict**: For mid-weight models like Qwen 35B, Agentic Code Generation must follow a two-step pattern:
1. **Scope Limitation**: Never ask for more than 30-40 objects per prompt.
2. **Lenient Parsing**: Never expect the LLM to output perfect syntax. Always use robust Python AST extraction scripts to map the LLM's "hallucinated syntax" back into the rigid AST structure.
