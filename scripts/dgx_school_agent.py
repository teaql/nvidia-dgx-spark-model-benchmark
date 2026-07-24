import urllib.request
import json
import time
import os
import subprocess
import re

url = "http://222.128.77.232:30704/v1/chat/completions"

def call_dgx(messages):
    payload = {
        "model": "nemotron-3-super",
        "messages": messages,
        "max_tokens": 8192,
        "temperature": 0.0,
        "chat_template_kwargs": {
            "enable_thinking": True,
            "reasoning_budget": 2048,
        },
    }
    
    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=600) as response:
        result = json.loads(response.read().decode("utf-8"))
        content = result["choices"][0]["message"]["content"]
        reasoning = result["choices"][0]["message"].get("reasoning_content", "")
        return content, reasoning

def extract_xml(content):
    if "<ksml" in content and "</ksml>" in content:
        start = content.find("<ksml")
        end = content.find("</ksml>") + 7
        return content[start:end]
    return content.strip()

def run_agent():
    system_prompt = """You are an autonomous AI Agent powered by Nemotron. Your task is to build a TeaQL KSML model.
    CRITICAL RULES:
    1. Output MUST be pure XML enclosed in <ksml data_service="sqlite">...</ksml>.
    2. Do NOT use <object> or <field> tags! Use entity names directly as tags (e.g. <school>).
    3. Use snake_case for entity tags and attribute names.
    4. Do not output markdown code blocks, just the XML.
    5. No fields named "type". Use "school_type" instead.
    """
    
    user_prompt = """Use Rust to build a school management system with these domain concepts:
- Platform
- School
- School Type, with values Primary and Secondary
Create the semantic TeaQL model first."""
    
    messages = [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": user_prompt}
    ]
    
    for iteration in range(1, 6):
        print(f"--- Iteration {iteration} ---")
        print("Waiting for DGX Nemotron to generate...")
        start_time = time.time()
        
        try:
            content, reasoning = call_dgx(messages)
        except Exception as e:
            print(f"API Error: {e}")
            break
            
        print(f"Generation took {time.time() - start_time:.2f}s")
        
        xml_content = extract_xml(content)
        
        # Save artifacts
        with open(f"artifacts/school-test/iter{iteration}_reasoning.txt", "w") as f:
            f.write(reasoning)
        with open(f"artifacts/school-test/model.xml", "w") as f:
            f.write(xml_content)
            
        # Run Evaluate
        print("Running cargo teaql evaluate...")
        eval_proc = subprocess.run(["cargo", "teaql", "--input", "artifacts/school-test/model.xml", "evaluate"], capture_output=True, text=True)
        eval_output = eval_proc.stdout + eval_proc.stderr
        
        with open(f"artifacts/school-test/iter{iteration}_eval.log", "w") as f:
            f.write(eval_output)
            
        # Check for Errors
        if "Errors: 0" in eval_output or "- **Errors**: 0" in eval_output:
            print("SUCCESS! 0 Errors found.")
            break
        else:
            print("Errors found. Sending back to Nemotron for fixing...")
            # Extract error summary lines to avoid sending back a massive log
            error_lines = []
            capture = False
            for line in eval_output.splitlines():
                if "| Count | Pattern |" in line or "| Rule ID |" in line:
                    capture = True
                if capture:
                    error_lines.append(line)
            
            error_summary = "\n".join(error_lines) if error_lines else eval_output[:1000]
            
            messages.append({"role": "assistant", "content": content})
            messages.append({"role": "user", "content": f"Your KSML model failed evaluation. Here are the errors:\n{error_summary}\n\nPlease fix these errors and output the complete, corrected XML."})
            
    # Once loop is done, try to generate Rust code
    if os.path.exists("artifacts/school-test/model.xml"):
        print("Generating Rust Core Library...")
        subprocess.run(["cargo", "teaql", "--input", "artifacts/school-test/model.xml", "generate", "rust-lib-core", "--output", "artifacts/school-test/rust-lib-core"])
        print("Generating Rust App Console...")
        subprocess.run(["cargo", "teaql", "--input", "artifacts/school-test/model.xml", "generate", "rust-app-console", "--output", "artifacts/school-test/rust-app-console"])
        print("Agent loop finished!")

if __name__ == "__main__":
    run_agent()
