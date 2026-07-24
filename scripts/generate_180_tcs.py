import os

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

def generate_tcs():
    lines = []
    lines.append("# Root Configuration")
    lines.append("SET_APP_NAME moving-company-service")
    lines.append("SET_DATA_SERVICE sqlite")
    lines.append("")
    
    total_objects = 0
    for module, entities in modules.items():
        lines.append(f"##############################################")
        lines.append(f"# MODULE: {module}")
        lines.append(f"##############################################")
        lines.append("")
        
        for entity in entities:
            lines.append(f"DEF_ENTITY {entity}")
            lines.append(f"SET_ATTR {entity} _module_key \"{module}\"")
            lines.append(f"SET_ATTR {entity} id 0")
            lines.append(f"SET_ATTR {entity} name \"Unknown\"")
            
            if entity not in ["platform", "merchant"]:
                lines.append(f"DEF_RELATION {entity} merchant_id merchant")
            
            lines.append("")
            total_objects += 1

    lines.append(f"# Total Objects Generated: {total_objects}")
    
    output_path = "artifacts/school-test/moving_company_180.tcs"
    with open(output_path, "w") as f:
        f.write("\n".join(lines))
    print(f"Generated {total_objects} objects to {output_path}")

if __name__ == "__main__":
    generate_tcs()
