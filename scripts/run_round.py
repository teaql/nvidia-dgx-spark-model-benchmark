import os
import sys
import json
import urllib.request
import time
import subprocess
import argparse
import re
import shutil
from concurrent.futures import ThreadPoolExecutor, as_completed

def call_dgx(messages, max_tokens=16384):
    url = "http://222.128.77.232:30704/v1/chat/completions"
    payload = {
        "model": "nemotron-3-super",
        "messages": messages,
        "max_tokens": max_tokens,
        "temperature": 0.0,
        "chat_template_kwargs": {
            "enable_thinking": True,
            "reasoning_budget": 2048,
        },
    }
    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=600) as response:
        result = json.loads(response.read().decode("utf-8"))
        return result["choices"][0]["message"]["content"]

def extract_xml(content):
    if "```xml" in content:
        start = content.find("```xml") + 6
        end = content.find("```", start)
        if end != -1:
            return content[start:end].strip()
    return content.strip()

def run_round(round_num, scale):
    out_dir = f"artifacts/round-{round_num}"
    
    for attempt in range(1, 4):
        print(f"--- Round {round_num} (Scale: {scale}) - Attempt {attempt} ---")
        if os.path.exists(out_dir):
            shutil.rmtree(out_dir)
            
        modular_dir = f"{out_dir}/modular"
        os.makedirs(modular_dir, exist_ok=True)
        
        print("Planning modules (Deterministic Python Allocation)...")
    
        base_entities = [
            "platform", "platform_config", "tenant_registry", "merchant", "branch", "franchise",
            "move_order", "move_quote", "route", "route_stop", "time_slot", "fulfillment_event", 
            "address", "crew", "dispatch_assignment", "damage_report", "proof_of_delivery",
            "packing_list", "inventory_item", "vehicle_load_plan", "weigh_station_ticket",
            "toll_receipt", "parking_permit", "traffic_violation", "detour_log", "fuel_stop",
            "weather_delay", "customer_signature", "walkthrough_checklist", "post_move_survey",
            "operations_manager_override", "employee", "department", "job_assignment", "work_shift",
            "worked_hours", "payroll_period", "payroll_calculation", "payslip", "bonus", "leave_request",
            "employee_certification", "tax_withholding", "direct_deposit_info", "union_dues",
            "overtime_approval", "expense_reimbursement", "performance_review", "warning_letter",
            "termination_record", "emergency_contact", "uniform_assignment", "background_check",
            "customer", "private_customer_profile", "corporate_customer_profile", "customer_contact",
            "billing_profile", "customer_history", "customer_preference", "customer_consent",
            "referral_code", "loyalty_tier", "complaint_ticket", "resolution_offer",
            "vip_status", "do_not_contact_list", "customer_note", "communication_log",
            "product", "service", "moving_service", "cleaning_service", "box_rental",
            "service_configuration", "price_list", "service_price", "service_bundle",
            "storage_unit", "packing_material", "insurance_addon", "piano_handling",
            "stair_fee", "long_carry_fee", "hoisting_service", "vehicle_transport",
            "pet_relocation_service", "campaign", "discount_code", "lead", "sales_opportunity",
            "lead_activity", "conversion_event", "conversion_metric", "ad_spend", "social_media_post",
            "email_blast", "sms_campaign", "sales_script", "objection_handling_guide",
            "competitor_analysis", "sales_territory", "payment", "invoice", "invoice_line", "refund",
            "expense", "vat_rate", "journal_entry", "account", "financial_summary", "tax_document",
            "bank_transaction", "merchant_fee", "chargeback_record", "credit_note",
            "debit_note", "audit_adjustment", "fiscal_year", "vehicle", "equipment", "consumable",
            "asset_assignment", "asset_inspection", "maintenance_schedule", "maintenance_event",
            "fuel_record", "supplier", "gps_tracker", "dashcam_footage", "tire_replacement",
            "oil_change_log", "registration_renewal", "insurance_card", "depreciation_schedule",
            "scrap_record", "contract", "insurance_policy", "insurance_claim", "document",
            "document_version", "compliance_check", "data_retention_policy", "recovery_request",
            "nda_agreement", "terms_of_service", "privacy_policy", "cookie_consent", "gdpr_request",
            "osha_incident", "system_user", "role", "permission", "user_role_assignment",
            "role_permission", "login_history", "user_session", "security_config", "audit_report",
            "system_log", "activity_log", "audit_log", "entity_change", "change_set", "login_attempt",
            "failed_auth_log", "notification", "notification_template", "automation_rule",
            "automation_trigger", "automation_action", "sms_delivery_receipt", "email_bounce_log",
            "api_client", "api_endpoint", "webhook", "webhook_delivery", "integration_mapping",
            "sync_job", "api_rate_limit"
        ]
        
        entities = []
        for i in range(scale):
            if i < len(base_entities):
                entities.append(base_entities[i])
            else:
                entities.append(f"custom_entity_{i}")
                
        modules = {}
        if scale <= 20:
            modules["main"] = entities
        else:
            for i in range(0, len(entities), 15):
                mod_name = f"module_{i//15}"
                modules[mod_name] = entities[i:i+15]
            
        print(f"Plan generated: {sum(len(v) for v in modules.values())} objects across {len(modules)} modules.")
        
        system_rules = open("/home/philip/githome/teaql-agent-kit/prompts/modeling/ksml-rules.md").read()
        system_prompt = f"""You are a KSML code generator. Strictly follow the KSML rules below.
IMPORTANT: Business objects MUST NEVER have 'id' or 'name' attributes defined explicitly! Do not output empty name="" or id="". Just omit them completely.
Every XML file must be wrapped in a <root> container.
CRITICAL: Do NOT use reserved keywords like 'move', 'type', 'match', 'async' as object names or attribute names.

KSML RULES:
{system_rules[:15000]}
"""
        
        def generate_module(mod_name, entities):
            user_prompt = f"Generate the KSML XML for the module '{mod_name}' containing exactly these entities: {', '.join(entities)}. Ensure it is wrapped in a <root> tag. Output ONLY valid XML inside ```xml ```."
            for module_attempt in range(1, 4):
                try:
                    content = call_dgx([
                        {"role": "system", "content": system_prompt},
                        {"role": "user", "content": user_prompt}
                    ])
                    xml = extract_xml(content)
                    
                    # AST Cleanup
                    xml = re.sub(r'\s+name=""', '', xml)
                    xml = re.sub(r'\s+id=""', '', xml)
                    xml = re.sub(r'\s+name="Unknown"', '', xml)
                    xml = re.sub(r'\s+id="0"', '', xml)
                    
                    with open(f"{modular_dir}/{mod_name}.xml", "w") as f:
                        f.write(xml)
                    return mod_name
                except Exception as e:
                    print(f"Module {mod_name} generation failed on attempt {module_attempt}: {e}")
                    time.sleep(5)
            raise Exception(f"Module {mod_name} failed to generate after 3 attempts.")
            
        if scale > 20 or len(modules) > 1:
            main_xml = [
                '<root alias_model_name="main_module"',
                '      english_name="Main Module"',
                '      chinese_name="主要模块"',
                '      name="main-service"',
                '      org="doublechaintech"',
                '      data_service="sqlite"',
                '      cfg_mask_china_mobile="false"',
                '      _module_key="root">'
            ]
            for m in modules.keys():
                if m != "main":
                    main_xml.append(f'  <_include file="{m}.xml" />')
            main_xml.append('</root>')
            with open(f"{modular_dir}/main.xml", "w") as f:
                f.write("\n".join(main_xml))
                
            with ThreadPoolExecutor(max_workers=2) as executor:
                futures = [executor.submit(generate_module, m, ents) for m, ents in modules.items() if m != "main"]
                module_failed = False
                for f in as_completed(futures):
                    try:
                        print(f"Generated module: {f.result()}")
                    except Exception as e:
                        print(f"Module generation failed with exception: {e}")
                        module_failed = True
                if module_failed:
                    print(f"Attempt {attempt} failed during module generation.")
                    continue
        else:
            main_name = list(modules.keys())[0]
            generate_module(main_name, list(modules.values())[0])
            if main_name != "main":
                os.rename(f"{modular_dir}/{main_name}.xml", f"{modular_dir}/main.xml")
            
        print("Evaluating...")
        res = subprocess.run(["cargo", "teaql", "--input", modular_dir, "evaluate"], capture_output=True, text=True)
        if "Errors" in res.stdout and " 0\n" not in res.stdout.split("Errors")[1][:10]:
            print(f"Evaluation Failed on attempt {attempt}!")
            print(res.stdout)
            continue
            
        print("Generating code...")
        subprocess.run(["cargo", "teaql", "--input", modular_dir, "rust-lib-core", "--output", f"{out_dir}/rust-lib-core"])
        subprocess.run(["unzip", "-o", "domain.zip"], cwd=f"{out_dir}/rust-lib-core", capture_output=True)
        subprocess.run(["cargo", "teaql", "--input", modular_dir, "rust-app-console", "--output", f"{out_dir}/rust-app-console"])
        subprocess.run(["unzip", "-o", "domain.zip"], cwd=f"{out_dir}/rust-app-console", capture_output=True)
        
        print("Compiling (cargo check)...")
        res = subprocess.run(["cargo", "check"], cwd=f"{out_dir}/rust-app-console", capture_output=True, text=True)
        if res.returncode != 0:
            print(f"Compilation Failed on attempt {attempt}!")
            print(res.stderr)
            continue
            
        report = f"""# Round {round_num} - Scale: {scale} Objects

## Summary
Successfully generated {scale} objects across {len(modules)} modules.

## Environment & Versions
- **Agent**: Antigravity 2.0
- **LLM**: DGX nemotron-3-super (128K context, max_tokens=16384)
- **TeaQL CLI**: 2.0.8
- **Rust Compiler**: 1.96.0

## Token Economy Analysis
Using modular generation with KSML `<_include>` allows the LLM to only process 10-20 objects at a time.
This bypasses long-context degradation and significantly reduces the input/output token overhead compared to a single massive XML payload.
"""
        with open(f"reports/Round-{round_num}-Report.md", "w") as f:
            f.write(report)
            
        print(f"Round {round_num} SUCCESS!")
        return True
        
    print(f"Round {round_num} FAILED after 3 attempts.")
    return False

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--round", type=int, required=True)
    parser.add_argument("--scale", type=int, required=True)
    args = parser.parse_args()
    success = run_round(args.round, args.scale)
    if not success:
        sys.exit(1)
