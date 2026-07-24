#!/usr/bin/env python3
"""Validate the fixed Round 2 TeaQL school-platform modeling task."""

from __future__ import annotations

import argparse
import json
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


BUSINESS_OBJECTS = {
    "platform",
    "school",
    "campus",
    "prospective_student",
    "admission_application",
    "guardian",
    "teacher",
    "course",
    "enrollment",
}
CONSTANT_OBJECTS = {
    "school_type",
    "application_status",
    "course_category",
    "enrollment_status",
}
EXPECTED_OBJECTS = BUSINESS_OBJECTS | CONSTANT_OBJECTS
EXPECTED_RELATIONSHIPS = {
    "school": {"platform": "platform()", "school_type": "school_type()"},
    "campus": {"school": "school()"},
    "prospective_student": {"school": "school()"},
    "admission_application": {
        "prospective_student": "prospective_student()",
        "school": "school()",
        "application_status": "application_status()",
    },
    "guardian": {"prospective_student": "prospective_student()"},
    "teacher": {"school": "school()"},
    "course": {
        "school": "school()",
        "teacher": "teacher()",
        "course_category": "course_category()",
    },
    "enrollment": {
        "prospective_student": "prospective_student()",
        "course": "course()",
        "enrollment_status": "enrollment_status()",
    },
}
EXPECTED_CONSTANT_VALUES = {
    "school_type": ["PRIMARY", "SECONDARY"],
    "application_status": ["SUBMITTED", "UNDER_REVIEW", "ACCEPTED", "REJECTED"],
    "course_category": ["REQUIRED", "ELECTIVE"],
    "enrollment_status": ["ENROLLED", "COMPLETED", "WITHDRAWN"],
}
ROOT_REQUIRED = {
    "alias_model_name",
    "cfg_mask_china_mobile",
    "chinese_name",
    "english_name",
    "data_service",
    "name",
    "org",
    "_module_key",
}
PROGRAMMING_KEYWORDS = {
    "async",
    "await",
    "class",
    "enum",
    "interface",
    "match",
    "module",
    "package",
    "type",
    "yield",
}
SQL_KEYWORDS = {"from", "group", "order", "select", "table", "user", "where"}
SNAKE_CASE = re.compile(r"^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$")
KEBAB_SERVICE = re.compile(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*-service$")
MODULE_KEY = re.compile(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$")
CONSTANT_CODE = re.compile(r"^[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)*$")
REFERENCE = re.compile(r"^[a-z][a-z0-9_]*\(\)$")


def validate_text(xml_text: str) -> dict:
    errors: list[str] = []
    warnings: list[str] = []
    stripped = xml_text.strip()

    if stripped.startswith("```") or stripped.endswith("```"):
        errors.append("Output contains Markdown fences.")
    if not stripped.startswith("<root"):
        errors.append("Document does not start with <root>.")

    try:
        root = ET.fromstring(stripped)
    except ET.ParseError as exc:
        return {
            "valid": False,
            "errors": errors + [f"XML parse error: {exc}"],
            "warnings": warnings,
            "object_count": 0,
        }

    if root.tag != "root":
        errors.append(f"Document element is <{root.tag}>, expected <root>.")

    missing_root = sorted(ROOT_REQUIRED - set(root.attrib))
    if missing_root:
        errors.append(f"Root missing attributes: {', '.join(missing_root)}.")
    if not root.attrib.get("name"):
        errors.append("Root name is empty.")
    elif not KEBAB_SERVICE.fullmatch(root.attrib["name"]):
        errors.append("Root name must be kebab-case ending in -service.")
    if root.attrib.get("data_service") != "sqlite":
        errors.append('Root data_service must be "sqlite".')
    if root.attrib.get("org") != "example":
        errors.append('Root org must be "example".')
    if root.attrib.get("_module_key") != "root":
        errors.append('Root _module_key must be "root".')
    if root.attrib.get("cfg_mask_china_mobile") != "false":
        errors.append('Root cfg_mask_china_mobile must be "false".')

    objects = list(root)
    names = [obj.tag for obj in objects]
    if len(names) != 13:
        errors.append(f"Expected 13 direct child objects, found {len(names)}.")
    if len(names) != len(set(names)):
        errors.append("Object definitions are not unique.")
    missing_objects = sorted(EXPECTED_OBJECTS - set(names))
    extra_objects = sorted(set(names) - EXPECTED_OBJECTS)
    if missing_objects:
        errors.append(f"Missing objects: {', '.join(missing_objects)}.")
    if extra_objects:
        errors.append(f"Unexpected objects: {', '.join(extra_objects)}.")

    by_name = {obj.tag: obj for obj in objects}
    for obj in objects:
        if not SNAKE_CASE.fullmatch(obj.tag):
            errors.append(f"{obj.tag}: object name is not lowercase snake_case.")
        if obj.tag in PROGRAMMING_KEYWORDS:
            errors.append(f"{obj.tag}: object name is a reserved keyword.")
        for required in ("_name", "_module", "_module_key"):
            if not obj.attrib.get(required):
                errors.append(f"{obj.tag}: missing {required}.")
        module_key = obj.attrib.get("_module_key")
        if module_key and not MODULE_KEY.fullmatch(module_key):
            errors.append(f"{obj.tag}: _module_key is not kebab-case.")
        for field_name in obj.attrib:
            if field_name in PROGRAMMING_KEYWORDS or field_name in SQL_KEYWORDS:
                errors.append(f"{obj.tag}.{field_name}: reserved attribute name.")
            if field_name.endswith("_id"):
                errors.append(f"{obj.tag}.{field_name}: relationship uses _id suffix.")

    for name in BUSINESS_OBJECTS:
        obj = by_name.get(name)
        if obj is None:
            continue
        for forbidden in ("id", "_constant", "_identifier"):
            if forbidden in obj.attrib:
                errors.append(f"{name}: business object contains {forbidden}.")
        if list(obj):
            errors.append(f"{name}: business object has nested elements.")
        if obj.attrib.get("create_time") != "createTime()":
            errors.append(f"{name}: missing create_time=\"createTime()\".")
        if obj.attrib.get("update_time") != "updateTime()":
            errors.append(f"{name}: missing update_time=\"updateTime()\".")
        if name == "platform":
            refs = [value for value in obj.attrib.values() if REFERENCE.fullmatch(value)]
            if refs:
                errors.append("platform: domain root references another object.")
        for field, expected in EXPECTED_RELATIONSHIPS.get(name, {}).items():
            if obj.attrib.get(field) != expected:
                errors.append(f'{name}: expected {field}="{expected}".')

    for name in CONSTANT_OBJECTS:
        obj = by_name.get(name)
        if obj is None:
            continue
        required_values = {
            "id": "id()",
            "name": "string()",
            "code": "string()",
            "_constant": "true",
            "_identifier": "code",
            "platform": "platform()",
        }
        for field, expected in required_values.items():
            if obj.attrib.get(field) != expected:
                errors.append(f'{name}: expected {field}="{expected}".')
        children = list(obj)
        if any(child.tag != "_value" for child in children):
            errors.append(f"{name}: contains nested elements other than _value.")
        expected_codes = EXPECTED_CONSTANT_VALUES[name]
        actual_codes = [child.attrib.get("code", "") for child in children]
        if actual_codes != expected_codes:
            errors.append(
                f"{name}: expected codes {expected_codes}, found {actual_codes}."
            )
        expected_ids = [str(1001 + i) for i in range(len(children))]
        actual_ids = [child.attrib.get("id", "") for child in children]
        if actual_ids != expected_ids:
            errors.append(
                f"{name}: expected sequential ids {expected_ids}, found {actual_ids}."
            )
        for child in children:
            if set(child.attrib) != {"id", "name", "code"}:
                errors.append(f"{name}: _value must contain only id, name, and code.")
            if not child.attrib.get("name"):
                errors.append(f"{name}: _value has empty name.")
            code = child.attrib.get("code", "")
            if code and not CONSTANT_CODE.fullmatch(code):
                errors.append(f"{name}: invalid constant code {code!r}.")

    return {
        "valid": not errors,
        "errors": errors,
        "warnings": warnings,
        "object_count": len(objects),
        "business_object_count": sum(name in BUSINESS_OBJECTS for name in names),
        "constant_object_count": sum(name in CONSTANT_OBJECTS for name in names),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("xml_file", type=Path)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    result = validate_text(args.xml_file.read_text(encoding="utf-8"))
    if args.json:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    else:
        print("PASS" if result["valid"] else "FAIL")
        for error in result["errors"]:
            print(f"- {error}")
    return 0 if result["valid"] else 1


if __name__ == "__main__":
    sys.exit(main())
