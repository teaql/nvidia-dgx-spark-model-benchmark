import xml.etree.ElementTree as ET
import sys

tree = ET.parse(sys.argv[1])
root = tree.getroot()
gender_type = ET.Element("gender_type", {
    "_name": "Gender Type",
    "_module": "Employees & Payroll",
    "_module_key": "employees-payroll",
    "_constant": "true",
    "platform": "platform()",
    "id": "id()",
    "name": "string()",
    "code": "string()",
    "_identifier": "code"
})
ET.SubElement(gender_type, "_value", {"id": "1001", "name": "Male", "code": "MALE"})
ET.SubElement(gender_type, "_value", {"id": "1002", "name": "Female", "code": "FEMALE"})
root.append(gender_type)
tree.write(sys.argv[1], encoding="utf-8", xml_declaration=True)
