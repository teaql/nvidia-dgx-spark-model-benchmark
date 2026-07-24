import xml.etree.ElementTree as ET
import sys
import re

try:
    tree = ET.parse(sys.argv[1])
    root = tree.getroot()
    
    # Ensure root has required attributes
    root.attrib["data_service"] = "sqlite"
    root.attrib["name"] = "moving-company-service"
    root.attrib["alias_model_name"] = "moving_company_management"

    for obj in root:
        if obj.tag != "constant":
            # For every child like <field name="X"> or <attribute name="X">
            children_to_remove = []
            for child in obj:
                if child.tag in ("attribute", "field", "property"):
                    attr_name = child.attrib.get("name")
                    if attr_name:
                        obj.attrib[attr_name] = "string()"
                    children_to_remove.append(child)
                elif child.tag in ("reference", "relation", "foreign_key"):
                    ref_name = child.attrib.get("name")
                    target = child.attrib.get("target") or child.attrib.get("ref") or "unknown"
                    if ref_name:
                        obj.attrib[ref_name] = f"{target}()"
                    children_to_remove.append(child)
            
            for c in children_to_remove:
                obj.remove(c)

    # Write out the file
    xml_str = ET.tostring(root, encoding="utf-8").decode("utf-8")
    
    # Apply keyword fixes
    xml_str = re.sub(r'\btype="', 'record_type="', xml_str)
    xml_str = xml_str.replace('<access_token ', '<access_token _audit_mask_fields="token_id" ')
    
    with open(sys.argv[1], 'w', encoding='utf-8') as f:
        f.write('<?xml version="1.0" encoding="utf-8"?>\n' + xml_str)

    print("Transformation successful.")
except Exception as e:
    print(f"Error: {e}")
