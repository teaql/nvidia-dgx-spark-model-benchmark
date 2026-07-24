import urllib.request
import json
import time
import os
from concurrent.futures import ThreadPoolExecutor, as_completed

url = "http://222.128.77.232:30704/v1/chat/completions"

modules = {
    "operations-logistics": "Operations & Logistics (25 objects): trucks, dispatch, routes, schedules, etc.",
    "employees-payroll": "Employees & Payroll (22 objects): staff, roles, salaries, attendance, etc.",
    "customer-management": "Customer Management (16 objects): users, profiles, addresses, preferences, etc.",
    "finance-accounting": "Finance & Accounting (17 objects): invoices, payments, taxes, refunds, etc.",
    "identity-access": "Identity & Access (10 objects): auth tokens, sessions, permissions, etc."
}

def call_dgx(messages):
    payload = {
        "model": "nemotron-3-super",
        "messages": messages,
        "max_tokens": 16384,
        "temperature": 0.0,
        "chat_template_kwargs": {
            "enable_thinking": True,
            "reasoning_budget": 2048,
        },
    }
    
    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=1200) as response:
        result = json.loads(response.read().decode("utf-8"))
        content = result["choices"][0]["message"]["content"]
        return content

def extract_xml(content):
    if "```xml" in content:
        start = content.find("```xml") + 6
        end = content.find("```", start)
        if end != -1:
            return content[start:end].strip()
    return content.strip()

def generate_module(module_key, module_desc):
    system_prompt = """You are a KSML modeling expert for the DGX Spark Agent workflow. 
Your task is to generate ONE included XML file for a modular TeaQL KSML model.
CRITICAL RULES from KSML-RULES.md:
1. Every included XML file MUST be a well-formed document wrapped in a <root> container.
2. DO NOT use <object> or <field> tags! Use the entity name as the tag (e.g., <employee> or <truck>).
3. Each entity must have id="0" and name="Unknown" (or equivalent literal).
4. No fields named "type".
5. Output pure XML."""
    
    user_prompt = f"""Generate the KSML XML file for this specific module:
{module_desc}

Ensure you generate EXACTLY the requested number of unique domain objects.
Remember to wrap all generated objects in a <root> container."""
    
    messages = [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": user_prompt}
    ]
    
    print(f"[{module_key}] Requesting from DGX Nemotron...")
    start = time.time()
    try:
        content = call_dgx(messages)
        xml_content = extract_xml(content)
        
        # Ensure output directory exists
        os.makedirs("artifacts/round-10/modular", exist_ok=True)
        file_path = f"artifacts/round-10/modular/{module_key}.xml"
        with open(file_path, "w") as f:
            f.write(xml_content)
            
        print(f"[{module_key}] Saved in {time.time()-start:.2f}s")
        return True
    except Exception as e:
        print(f"[{module_key}] Error: {e}")
        return False

def run_round10():
    print("Starting DGX Spark Modular Agent for Round 10 (90 Objects)...")
    os.makedirs("artifacts/round-10/modular", exist_ok=True)
    
    # Generate main.xml
    main_xml = []
    main_xml.append('<root name="moving-company-service" data_service="sqlite">')
    for module in modules.keys():
        main_xml.append(f'  <_include file="{module}.xml" />')
    main_xml.append('</root>')
    
    with open("artifacts/round-10/modular/main.xml", "w") as f:
        f.write("\n".join(main_xml))
    print("Generated main.xml")
    
    # Concurrently generate the 5 modules
    with ThreadPoolExecutor(max_workers=5) as executor:
        futures = []
        for key, desc in modules.items():
            futures.append(executor.submit(generate_module, key, desc))
            
        for future in as_completed(futures):
            pass # wait for all
            
    print("\nAll modules generated! Now evaluating via cargo teaql...")
    
if __name__ == "__main__":
    run_round10()
