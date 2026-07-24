#!/usr/bin/env python3
"""Migrate the canonical moving-company seed to the Round 5 KSML contract."""

from __future__ import annotations

import argparse
import importlib.util
import xml.etree.ElementTree as ET
from pathlib import Path


OLD_MODULE_MAP = {
    "operations_and_logistics": "operations-logistics",
    "employee_and_payroll": "employees-payroll",
    "customer_management": "customer-management",
    "products_and_services": "products-services",
    "marketing_and_sales": "marketing-sales",
    "finance_and_accounting": "finance-accounting",
    "asset_management": "asset-management",
    "administration_and_compliance": "administration-compliance",
    "user_and_role_management": "identity-access",
    "platform_data": "platform-administration",
    "platform": "platform-administration",
    "organization": "organization-administration",
}

EXPLICIT_MODULES = {
    "platform": "platform-administration",
    "platform_configuration": "platform-administration",
    "platform_locale": "platform-administration",
    "merchant": "organization-administration",
    "merchant_branch": "organization-administration",
    "merchant_setting": "organization-administration",
    "user_account": "identity-access",
    "user_role": "identity-access",
    "role": "identity-access",
    "permission": "identity-access",
    "permission_set": "identity-access",
    "user_role_assignment": "identity-access",
    "role_permission": "identity-access",
    "magic_link": "identity-access",
    "user_session": "identity-access",
    "authentication_attempt": "identity-access",
    "auth_method_type": "identity-access",
    "audit_log": "activity-audit",
    "activity_log": "activity-audit",
    "entity_change": "activity-audit",
    "change_set": "activity-audit",
    "version_history": "activity-audit",
    "deleted_record": "activity-audit",
    "action_type": "activity-audit",
    "notification": "notifications-automation",
    "notification_template": "notifications-automation",
    "notification_preference": "notifications-automation",
    "notification_delivery": "notifications-automation",
    "automation_rule": "notifications-automation",
    "automation_trigger": "notifications-automation",
    "automation_action": "notifications-automation",
    "system_automation_hook": "notifications-automation",
    "notification_channel": "notifications-automation",
    "api_client": "api-integrations",
    "api_endpoint": "api-integrations",
    "webhook": "api-integrations",
    "webhook_delivery": "api-integrations",
    "integration_mapping": "api-integrations",
    "synchronization_run": "api-integrations",
    "external_integration": "api-integrations",
}

SUPPLEMENTAL_NAMES = {
    "platform-administration": [
        "platform_configuration",
        "platform_locale",
        "platform_feature_flag",
    ],
    "organization-administration": [
        "merchant_branch",
        "merchant_setting",
        "merchant_operating_region",
    ],
    "operations-logistics": [
        "move_order",
        "move_quote",
        "route_stop",
        "crew",
        "dispatch_assignment",
        "damage_report",
        "proof_of_delivery",
        "operational_exception",
        "crew_member_assignment",
        "pickup_instruction",
        "delivery_instruction",
        "move_inventory",
    ],
    "employees-payroll": [
        "work_shift",
        "worked_hours",
        "payroll_period",
        "payslip",
        "bonus",
        "employee_certification",
        "employee_availability",
        "payroll_deduction",
        "training_session",
        "shift_assignment",
    ],
    "customer-management": [
        "private_customer_profile",
        "corporate_customer_profile",
        "billing_profile",
        "customer_history",
        "customer_preference",
        "customer_consent",
        "customer_complaint",
        "customer_note",
    ],
    "products-services": [
        "service",
        "moving_service",
        "cleaning_service",
        "box_rental",
        "service_configuration",
        "service_price",
        "service_bundle",
        "storage_service",
        "packing_service",
        "disposal_service",
        "rental_period",
        "service_area",
    ],
    "marketing-sales": [
        "campaign",
        "sales_opportunity",
        "conversion_event",
        "campaign_audience",
        "campaign_channel",
        "lead_attribution",
        "sales_funnel",
    ],
    "finance-accounting": [
        "invoice_line",
        "refund",
        "expense",
        "vat_rate",
        "account",
        "financial_summary",
        "expense_claim",
        "settlement",
        "receivable",
        "payable",
    ],
    "asset-management": [
        "asset_assignment",
        "asset_inspection",
        "maintenance_event",
        "fuel_record",
        "supplier",
        "vehicle_inspection",
        "equipment_checkout",
        "consumable_reorder",
    ],
    "administration-compliance": [
        "contract",
        "insurance_claim",
        "document",
        "document_version",
        "compliance_check",
        "data_retention_policy",
        "recovery_request",
        "compliance_incident",
        "policy_acknowledgement",
        "regulatory_license",
        "safety_policy",
        "incident_report",
        "risk_assessment",
        "legal_case",
        "privacy_request",
        "document_retention_event",
        "insurance_coverage",
    ],
    "identity-access": [
        "role",
        "permission",
        "magic_link",
        "user_session",
        "authentication_attempt",
        "access_policy",
    ],
    "activity-audit": [
        "activity_log",
        "entity_change",
        "change_set",
        "audit_export",
    ],
    "notifications-automation": [
        "notification",
        "notification_preference",
        "notification_delivery",
        "automation_rule",
        "automation_trigger",
        "automation_action",
    ],
    "api-integrations": [
        "api_client",
        "api_endpoint",
        "webhook",
        "webhook_delivery",
        "integration_mapping",
        "synchronization_run",
    ],
}


def load_validator(path: Path):
    spec = importlib.util.spec_from_file_location("round5_validator", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load validator module from {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def display_name(tag: str) -> str:
    return tag.replace("_", " ").title()


def make_business_object(
    tag: str, module_key: str, modules: dict[str, tuple[str, int]]
) -> ET.Element:
    module_name = modules[module_key][0]
    return ET.Element(
        tag,
        {
            "_name": display_name(tag),
            "_module": module_name,
            "_module_key": module_key,
            "record_name": f"{display_name(tag)} Example",
            "merchant": "merchant(context)",
            "create_time": "createTime()",
            "update_time": "updateTime()",
        },
    )


def migrate(
    source: Path, output: Path, baseline_path: Path, validator_path: Path
) -> None:
    validator = load_validator(validator_path)
    modules = validator.MODULES
    required_anchors = validator.REQUIRED_ANCHORS

    root = ET.parse(source).getroot()
    root.attrib.clear()
    root.attrib.update(
        {
            "alias_model_name": "moving_company_management",
            "chinese_name": "Moving Company Management Platform",
            "english_name": "Moving Company Management Platform",
            "name": "moving-company-service",
            "cfg_mask_china_mobile": "false",
            "data_service": "sqlite",
            "org": "doublechaintech",
            "_module_key": "root",
        }
    )

    baseline = {
        obj.tag: ET.fromstring(ET.tostring(obj, encoding="unicode"))
        for obj in ET.parse(baseline_path).getroot()
    }
    original_module = {
        obj.tag: obj.attrib.get("_module", "") for obj in list(root)
    }

    for index, obj in enumerate(list(root)):
        if obj.tag in baseline:
            root.remove(obj)
            root.insert(index, baseline[obj.tag])

    for obj in list(root):
        if obj.tag in baseline:
            continue
        old_module = original_module.get(obj.tag, obj.attrib.get("_module", ""))
        module_key = EXPLICIT_MODULES.get(
            obj.tag, OLD_MODULE_MAP.get(old_module, "administration-compliance")
        )
        obj.attrib["_module"] = modules[module_key][0]
        obj.attrib["_module_key"] = module_key

        if obj.attrib.get("_constant") == "true":
            obj.attrib.pop("merchant", None)
            obj.attrib["platform"] = "platform()"
            for value_index, value in enumerate(obj.findall("_value"), start=1001):
                value.attrib["id"] = str(value_index)
        else:
            obj.attrib.setdefault("merchant", "merchant(context)")

        # These helpers are runtime request functions rather than model
        # references. Use concrete documentation values in the semantic seed.
        if obj.tag == "user_account":
            obj.attrib["remote_ip"] = "203.0.113.10"
            obj.attrib["city_by_ip"] = "Helsinki"
        elif obj.tag == "audit_log":
            obj.attrib["remote_ip"] = "203.0.113.10"

    by_name = {obj.tag: obj for obj in root}
    for module_key, anchors in required_anchors.items():
        for tag in sorted(anchors):
            if tag not in by_name:
                obj = make_business_object(tag, module_key, modules)
                root.append(obj)
                by_name[tag] = obj
            elif tag not in baseline:
                by_name[tag].attrib["_module"] = modules[module_key][0]
                by_name[tag].attrib["_module_key"] = module_key

    for module_key, (_, minimum) in modules.items():
        candidates = iter(SUPPLEMENTAL_NAMES[module_key])
        fallback_counter = 1
        while sum(
            1 for obj in root if obj.attrib.get("_module_key") == module_key
        ) < minimum:
            try:
                tag = next(candidates)
            except StopIteration:
                tag = f"extra_{module_key.replace('-', '_')}_{fallback_counter}"
                fallback_counter += 1
            if tag in by_name:
                continue
            obj = make_business_object(tag, module_key, modules)
            root.append(obj)
            by_name[tag] = obj

    ET.indent(root, space="  ")
    output.parent.mkdir(parents=True, exist_ok=True)
    ET.ElementTree(root).write(
        output, encoding="utf-8", xml_declaration=True, short_empty_elements=True
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("baseline", type=Path)
    parser.add_argument("validator", type=Path)
    args = parser.parse_args()
    migrate(args.source, args.output, args.baseline, args.validator)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
