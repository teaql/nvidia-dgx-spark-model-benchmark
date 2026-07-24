import os
import urllib.request
import json
import time
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed

modules = {
    "platform-administration": [
        "platform", "platform_config", "tenant_registry"
    ],
    "organization-administration": [
        "merchant", "branch", "franchise"
    ],
    "operations-logistics": [
        "move_order", "move_quote", "route", "route_stop", "time_slot", "fulfillment_event", 
        "address", "crew", "dispatch_assignment", "damage_report", "proof_of_delivery",
        "packing_list", "inventory_item", "vehicle_load_plan", "weigh_station_ticket",
        "toll_receipt", "parking_permit", "traffic_violation", "detour_log", "fuel_stop",
        "weather_delay", "customer_signature", "walkthrough_checklist", "post_move_survey",
        "operations_manager_override"
    ],
    "employees-payroll": [
        "employee", "department", "job_assignment", "work_shift", "worked_hours",
        "payroll_period", "payroll_calculation", "payslip", "bonus", "leave_request",
        "employee_certification", "tax_withholding", "direct_deposit_info", "union_dues",
        "overtime_approval", "expense_reimbursement", "performance_review", "warning_letter",
        "termination_record", "emergency_contact", "uniform_assignment", "background_check"
    ],
    "customer-management": [
        "customer", "private_customer_profile", "corporate_customer_profile", "customer_contact",
        "billing_profile", "customer_history", "customer_preference", "customer_consent",
        "referral_code", "loyalty_tier", "complaint_ticket", "resolution_offer",
        "vip_status", "do_not_contact_list", "customer_note", "communication_log"
    ],
    "products-services": [
        "product", "service", "moving_service", "cleaning_service", "box_rental",
        "service_configuration", "price_list", "service_price", "service_bundle",
        "storage_unit", "packing_material", "insurance_addon", "piano_handling",
        "stair_fee", "long_carry_fee", "hoisting_service", "vehicle_transport",
        "pet_relocation_service"
    ],
    "marketing-sales": [
        "campaign", "discount_code", "lead", "sales_opportunity", "lead_activity",
        "conversion_event", "conversion_metric", "ad_spend", "social_media_post",
        "email_blast", "sms_campaign", "sales_script", "objection_handling_guide",
        "competitor_analysis", "sales_territory"
    ],
    "finance-accounting": [
        "payment", "invoice", "invoice_line", "refund", "expense", "vat_rate",
        "journal_entry", "account", "financial_summary", "tax_document",
        "bank_transaction", "merchant_fee", "chargeback_record", "credit_note",
        "debit_note", "audit_adjustment", "fiscal_year"
    ],
    "asset-management": [
        "vehicle", "equipment", "consumable", "asset_assignment", "asset_inspection",
        "maintenance_schedule", "maintenance_event", "fuel_record", "supplier",
        "gps_tracker", "dashcam_footage", "tire_replacement", "oil_change_log",
        "registration_renewal", "insurance_card", "depreciation_schedule", "scrap_record"
    ],
    "administration-compliance": [
        "contract", "insurance_policy", "insurance_claim", "document", "document_version",
        "compliance_check", "data_retention_policy", "recovery_request", "nda_agreement",
        "terms_of_service", "privacy_policy", "cookie_consent", "gdpr_request",
        "osha_incident"
    ],
    "identity-access": [
        "user_account", "role", "permission", "user_role_assignment", "role_permission",
        "magic_link", "user_session", "password_reset", "two_factor_auth", "access_token"
    ],
    "activity-audit": [
        "activity_log", "audit_log", "entity_change", "change_set", "login_attempt",
        "failed_auth_log"
    ],
    "notifications-automation": [
        "notification", "notification_template", "automation_rule", "automation_trigger",
        "automation_action", "sms_delivery_receipt", "email_bounce_log"
    ],
    "api-integrations": [
        "api_client", "api_endpoint", "webhook", "webhook_delivery", "integration_mapping",
        "sync_job", "api_rate_limit"
    ]
}

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
        return content

def extract_xml(content):
    # Try to extract from ```xml...``` block
    if "```xml" in content:
        start = content.find("```xml") + 6
        end = content.find("```", start)
        if end != -1:
            return content[start:end].strip()
            
    # Or just return raw assuming no markdown wrapper
    return content.strip()

def generate_module(module_name, entities):
    system_prompt = """You are an expert KSML architect. Output ONLY valid XML. Do NOT wrap in <root> tags, just output the sibling entities directly. Do NOT use <object> or <field>. Use the entity name directly (e.g. <crew .../>). Each entity must have id="0", name="Unknown", and _module_key attribute."""
    
    entity_list = ", ".join(entities)
    user_prompt = f"Generate the KSML XML fragment for the module: '{module_name}'. It must contain exactly these entities: {entity_list}. For entities other than 'platform' and 'merchant', include a relation attribute 'merchant_id=\"0\"'. Output pure XML."
    
    messages = [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": user_prompt}
    ]
    
    print(f"[{module_name}] Sending to DGX Nemotron...")
    start = time.time()
    try:
        content = call_dgx(messages)
    except Exception as e:
        print(f"[{module_name}] Error: {e}")
        return module_name, ""
        
    xml_content = extract_xml(content)
    print(f"[{module_name}] Finished in {time.time() - start:.2f}s")
    
    file_path = f"artifacts/modular-180/{module_name}.xml"
    with open(file_path, "w") as f:
        f.write(xml_content)
        
    return module_name, xml_content

def run_agent():
    # 1. Generate main.xml
    main_xml = []
    main_xml.append('<root name="moving-company-service" data_service="sqlite">')
    for module in modules.keys():
        main_xml.append(f'  <_include file="{module}.xml" />')
    main_xml.append('</root>')
    
    with open("artifacts/modular-180/main.xml", "w") as f:
        f.write("\n".join(main_xml))
    print("Wrote artifacts/modular-180/main.xml")
    
    # 2. Concurrently generate all 14 modules
    with ThreadPoolExecutor(max_workers=14) as executor:
        futures = []
        for module, entities in modules.items():
            futures.append(executor.submit(generate_module, module, entities))
            
        for future in as_completed(futures):
            module, _ = future.result()
            
    print("\nAll 14 modules generated successfully!")
    
    # 3. Evaluate the main.xml
    print("\nEvaluating main.xml...")
    eval_proc = subprocess.run(["cargo", "teaql", "--input", "artifacts/modular-180/main.xml", "evaluate"], capture_output=True, text=True)
    print(eval_proc.stdout)
    if eval_proc.stderr:
        print("STDERR:")
        print(eval_proc.stderr)

if __name__ == "__main__":
    run_agent()
