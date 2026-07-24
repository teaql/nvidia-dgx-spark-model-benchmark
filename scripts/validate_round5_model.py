#!/usr/bin/env python3
"""Fixed structural validator for the Round 5 moving-company KSML task."""

from __future__ import annotations

import json
import re
import sys
import xml.etree.ElementTree as ET
from collections import Counter
from pathlib import Path


MODULES = {
    "platform-administration": ("Platform Administration", 3),
    "organization-administration": ("Organization Administration", 3),
    "operations-logistics": ("Operations & Logistics", 25),
    "employees-payroll": ("Employees & Payroll", 22),
    "customer-management": ("Customer Management", 16),
    "products-services": ("Products & Services", 18),
    "marketing-sales": ("Marketing & Sales", 15),
    "finance-accounting": ("Finance & Accounting", 17),
    "asset-management": ("Asset Management", 17),
    "administration-compliance": ("Administration & Compliance", 14),
    "identity-access": ("Identity & Access", 10),
    "activity-audit": ("Activity & Audit", 6),
    "notifications-automation": ("Notifications & Automation", 7),
    "api-integrations": ("API & Integrations", 7),
}

REQUIRED_ANCHORS = {
    "operations-logistics": {
        "move_order",
        "move_quote",
        "route",
        "route_stop",
        "time_slot",
        "fulfillment_event",
        "address",
        "crew",
        "dispatch_assignment",
        "damage_report",
        "proof_of_delivery",
    },
    "employees-payroll": {
        "employee",
        "department",
        "job_assignment",
        "work_shift",
        "worked_hours",
        "payroll_period",
        "payroll_calculation",
        "payslip",
        "bonus",
        "leave_request",
        "employee_certification",
    },
    "customer-management": {
        "customer",
        "private_customer_profile",
        "corporate_customer_profile",
        "customer_contact",
        "billing_profile",
        "customer_history",
        "customer_preference",
        "customer_consent",
    },
    "products-services": {
        "product",
        "service",
        "moving_service",
        "cleaning_service",
        "box_rental",
        "service_configuration",
        "price_list",
        "service_price",
        "service_bundle",
    },
    "marketing-sales": {
        "campaign",
        "discount_code",
        "lead",
        "sales_opportunity",
        "lead_activity",
        "conversion_event",
        "conversion_metric",
    },
    "finance-accounting": {
        "payment",
        "invoice",
        "invoice_line",
        "refund",
        "expense",
        "vat_rate",
        "journal_entry",
        "account",
        "financial_summary",
    },
    "asset-management": {
        "vehicle",
        "equipment",
        "consumable",
        "asset_assignment",
        "asset_inspection",
        "maintenance_schedule",
        "maintenance_event",
        "fuel_record",
        "supplier",
    },
    "administration-compliance": {
        "contract",
        "insurance_policy",
        "insurance_claim",
        "document",
        "document_version",
        "compliance_check",
        "data_retention_policy",
        "recovery_request",
    },
    "identity-access": {
        "user_account",
        "role",
        "permission",
        "user_role_assignment",
        "role_permission",
        "magic_link",
        "user_session",
    },
    "activity-audit": {
        "activity_log",
        "audit_log",
        "entity_change",
        "change_set",
    },
    "notifications-automation": {
        "notification",
        "notification_template",
        "automation_rule",
        "automation_trigger",
        "automation_action",
    },
    "api-integrations": {
        "api_client",
        "api_endpoint",
        "webhook",
        "webhook_delivery",
        "integration_mapping",
    },
}

ENGLISH_MODULE_RE = re.compile(r"^[A-Za-z0-9 &/()'-]+$")
KEBAB_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
PADDING_RE = re.compile(r"^(?:entity|object|item|record|module)_?\d+$")


def load_root(path: Path, errors: list[str]) -> ET.Element | None:
    try:
        root = ET.parse(path).getroot()
    except (OSError, ET.ParseError) as exc:
        errors.append(f"{path}: cannot parse XML: {exc}")
        return None
    if root.tag != "root":
        errors.append(f"{path}: document element must be <root>, got <{root.tag}>")
    return root


def main() -> int:
    if len(sys.argv) != 3:
        print(
            "usage: validate_round5_model.py MODEL_XML PRESERVED_OBJECTS_XML",
            file=sys.stderr,
        )
        return 2

    model_path = Path(sys.argv[1])
    baseline_path = Path(sys.argv[2])
    errors: list[str] = []
    model_root = load_root(model_path, errors)
    baseline_root = load_root(baseline_path, errors)
    if model_root is None or baseline_root is None:
        print(json.dumps({"valid": False, "errors": errors}, indent=2))
        return 1

    if model_root.attrib.get("name") != "moving-company-service":
        errors.append('root name must be "moving-company-service"')
    if model_root.attrib.get("alias_model_name") != "moving_company_management":
        errors.append(
            'root alias_model_name must be "moving_company_management"'
        )

    objects = list(model_root)
    names = [obj.tag for obj in objects]
    name_set = set(names)
    duplicate_names = sorted(name for name, count in Counter(names).items() if count > 1)

    if len(objects) < 180:
        errors.append(f"object count is {len(objects)}; expected at least 180")
    if duplicate_names:
        errors.append(f"duplicate direct-child objects: {duplicate_names}")

    padded = sorted(name for name in name_set if PADDING_RE.fullmatch(name))
    if padded:
        errors.append(f"count-padding object names are forbidden: {padded}")

    by_name = {obj.tag: obj for obj in objects}
    module_counts: Counter[str] = Counter()
    for obj in objects:
        display_name = obj.attrib.get("_name")
        module_name = obj.attrib.get("_module")
        module_key = obj.attrib.get("_module_key")
        if not display_name:
            errors.append(f"{obj.tag}: missing _name")
        if not module_name:
            errors.append(f"{obj.tag}: missing _module")
        elif not ENGLISH_MODULE_RE.fullmatch(module_name):
            errors.append(f"{obj.tag}: _module is not English: {module_name!r}")
        if not module_key:
            errors.append(f"{obj.tag}: missing _module_key")
        elif not KEBAB_RE.fullmatch(module_key):
            errors.append(f"{obj.tag}: _module_key is not kebab-case: {module_key!r}")
        if module_key:
            module_counts[module_key] += 1
            expected = MODULES.get(module_key)
            if expected is None:
                errors.append(f"{obj.tag}: unsupported workbench key {module_key!r}")
            elif module_name != expected[0]:
                errors.append(
                    f"{obj.tag}: module pair mismatch; {module_key!r} requires "
                    f"{expected[0]!r}, got {module_name!r}"
                )

    for module_key, (_, minimum) in MODULES.items():
        actual = module_counts[module_key]
        if actual < minimum:
            errors.append(
                f"{module_key}: has {actual} objects; expected at least {minimum}"
            )

    for module_key, anchors in REQUIRED_ANCHORS.items():
        missing = sorted(anchors - name_set)
        if missing:
            errors.append(f"{module_key}: missing required anchors {missing}")
        wrong_module = sorted(
            name
            for name in anchors & name_set
            if by_name[name].attrib.get("_module_key") != module_key
        )
        if wrong_module:
            errors.append(
                f"{module_key}: anchors assigned to another workbench {wrong_module}"
            )

    for baseline in baseline_root:
        actual = by_name.get(baseline.tag)
        if actual is None:
            errors.append(f"preserved object missing: {baseline.tag}")
        elif actual.attrib != baseline.attrib:
            errors.append(
                f"{baseline.tag}: definition differs from preserved baseline; "
                f"expected {baseline.attrib}, got {actual.attrib}"
            )

    summary = {
        "valid": not errors,
        "object_count": len(objects),
        "module_counts": dict(sorted(module_counts.items())),
        "preserved_objects": [
            obj.tag for obj in baseline_root if obj.tag in by_name
        ],
        "errors": errors,
    }
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
