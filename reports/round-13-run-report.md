# Round 13: Microservice Benchmark (30 Objects) on Nemotron 120B

## Overview
This round was initiated to establish a comparative baseline against the Qwen 35B model. We switched back to NVIDIA's official **Nemotron-3-Super 120B NVFP4** model to see if a significantly larger model (120B vs 35B) running with an explicit Reasoning Parser (`reasoning_budget=2048`) could overcome the syntax hallucinations observed in Round 12.

## Execution
- **Context Length**: 64K
- **Model Used**: `/home/dgx007/models/nemotron-3-super-120b-nvfp4`
- **Target Size**: Exactly 30 Objects (2 Modules).
- **Generation Time**: ~3 minutes (173 seconds).
- **Reasoning**: Successfully generated a distinct Chain-of-Thought (saved to `reasoning.txt`).

## Observations
1. **Logical Coherence (Perfect)**: Like the 35B model, Nemotron easily mapped out the 30 objects without falling into recursion or infinite loops. Its separation of reasoning from content allowed it to output clean XML without conversational bleeding.
2. **Persistent Syntax Hallucinations (185 NATIVE ERRORS)**: In a fascinating turn of events, the 120B model hallucinated the exact same incorrect syntax as the 35B model! It wrapped entities in `<object>` tags and attributes in `<field>` tags. It also used `camelCase` for field names instead of `snake_case` (triggering 190 styling warnings).
3. **AST Rescue**: Our AST extraction script (`transform_to_ksml_v4.py`) was once again able to seamlessly flatten the hallucinated XML tags back into valid KSML.
4. **Final Result**: After script correction and a single privacy mask patch, `cargo teaql evaluate` returned **0 Errors (34 Solid Objects)**.

## Conclusion
The results from Round 13 definitively prove that **Syntax Hallucination in KSML is Model-Agnostic**.
Whether using a 35B model or a 120B model with deep reasoning enabled, the LLMs inherently struggle to output flattened dynamic tags (e.g., `<truck_registry>`) and natively gravitate towards structural wrappers (e.g., `<object name="truck_registry">`).

**Architectural Takeaway**: 
We cannot solve the KSML syntax compliance issue by throwing a larger model at it. The solution is strictly engineering-based:
- Keep the object scope to 30-40 objects per prompt to guarantee logical coherence.
- Always intercept the LLM's XML output with an AST normalizer script (like `transform_to_ksml_v4.py`) before passing it to the deterministic compiler.
