import xml.etree.ElementTree as ET
import sys
import re

try:
    tree = ET.parse(sys.argv[1])
    old_root = tree.getroot()
    new_root = ET.Element("root")
    
    # Ensure root has required attributes
    new_root.attrib["data_service"] = "sqlite"
    new_root.attrib["name"] = "moving-company-service"
    new_root.attrib["alias_model_name"] = "moving_company_management"

    for obj in old_root:
        if obj.tag == "object":
            name = obj.attrib.get("name")
            if not name:
                continue
            new_obj = ET.Element(name)
            for k, v in obj.attrib.items():
                if k != "name":
                    new_obj.attrib[k] = v
            # process children
            for child in obj:
                if child.tag in ("attribute", "field", "property"):
                    attr_name = child.attrib.get("name")
                    if attr_name:
                        new_obj.attrib[attr_name] = "string()"
                elif child.tag in ("reference", "relation", "foreign_key"):
                    ref_name = child.attrib.get("name")
                    target = child.attrib.get("target") or child.attrib.get("ref") or "unknown"
                    if ref_name:
                        new_obj.attrib[ref_name] = f"{target}()"
            new_root.append(new_obj)
        else:
            new_root.append(obj)

    # Write out the file
    xml_str = ET.tostring(new_root, encoding="utf-8").decode("utf-8")
    
    # Apply keyword fixes
    xml_str = re.sub(r'\btype="', 'record_type="', xml_str)
    xml_str = xml_str.replace('<access_token ', '<access_token _audit_mask_fields="token_id" ')
    
    with open(sys.argv[1], 'w', encoding='utf-8') as f:
        f.write('<?xml version="1.0" encoding="utf-8"?>\n' + xml_str)

    print("Transformation successful.")
except Exception as e:
    print(f"Error: {e}")
