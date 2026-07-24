# Round 8: Run Report (Full 180-Object Monolith on Qwen 128K)

## Overview
This round evaluated the viability of pushing an LLM (`qwen3.6-35b-a3b` with a 128K context window) to its absolute limits by attempting to generate a massive, monolithic domain model consisting of **180+ objects**. The goal was to observe if modern large-context models could successfully handle enterprise-scale structural generation in a single shot without hallucination or architectural decay.

## Modeling Phase (The 128K Context Stress Test)
- **Target Context**: Full "Moving Company" Monolith
- **Object Count**: ~180
- **Model Used**: `/home/dgx007/models/qwen3.6-35b-a3b`
- **Output Size**: The model generated nearly 15,000 tokens of output and took approximately 12 minutes to stream the response.

### Observations of Model Degradation (The "Lost in the Middle" Effect)
Despite the powerful 128K context window, the model demonstrated severe cognitive decay when forced to output such a massive interconnected graph:
1. **Instruction Forgetting**: The model forgot the core KSML syntax tag structure (`<entity_name>`), reverting to generic XML formatting (`<object name="entity_name">`).
2. **CoT Bleeding**: The model leaked its "Chain of Thought" (CoT) directly into the output file before opening the XML tags, forcing us to write custom Python scripts to scrape and extract the valid payload.
3. **Massive Error Cascade**: When initially fed to `cargo teaql evaluate`, the raw output produced **732 Structural Errors**.
4. **Keyword Hallucinations**: Even after forcefully patching the XML structure via AST transformation scripts, the model hallucinated 14 reserved keyword conflicts (e.g., naming fields `type`), triggering `KSML-KEYWORD-002` warnings.

## Code Generation & Testing Phase
Although we successfully forced the broken XML through a series of Python transformation scripts and fixed the reserved keywords to reach 0 XML validation errors, the resulting Rust SDK generated highly inconsistent relation references (e.g., naming references `merchant_ref` instead of `merchant`). 

When we attempted to write the standard E-Expression integration tests against the SDK, the Rust compiler threw `[E0599]: no method named x found for struct y`, proving that the generated domain graph was semantically disorganized.

**Testing was aborted at this stage as per user instruction**, as the foundational model was too chaotic to provide meaningful business logic.

## Final Conclusion & Key Takeaways
1. **Large Context != Large Generation Capability**: While models can *read* 128K tokens effectively, forcing them to *generate* a 15,000+ token highly structured, inter-dependent graph in a single prompt leads to catastrophic syntax decay and hallucination. 
2. **Microservices vs. Monoliths**: This round completely validates our earlier architectural pivot. The AI is highly competent at generating **~30 object microservices** (100% success rate in Round 6 & 7) but collapses under the weight of a **180 object monolith**. 
3. **Agentic Workflows are Mandatory**: The fact that we even reached the Rust compilation phase for 180 broken objects was solely due to the automated `cargo teaql evaluate` engine intercepting 732 errors and the Agent auto-writing Python scripts to salvage the XML. Without an Agent and a rigid compilation evaluator, this output would be utterly unusable.

**Strategic Recommendation moving forward**: Never use a single LLM prompt to model a 100+ object enterprise system. Always use Domain-Driven Design (DDD) to break the system into <30 object Bounded Contexts, and generate them individually.
