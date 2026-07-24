import xml.etree.ElementTree as ET
import sys

try:
    tree = ET.parse(sys.argv[1])
    root = tree.getroot()
    new_root = ET.Element("root")
    for k, v in root.attrib.items():
        new_root.attrib[k] = v
        
    for obj in root:
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
                if child.tag == "attribute":
                    attr_name = child.attrib.get("name")
                    if attr_name:
                        new_obj.attrib[attr_name] = "string()"
                elif child.tag == "reference":
                    ref_name = child.attrib.get("name")
                    target = child.attrib.get("target")
                    if ref_name and target:
                        new_obj.attrib[ref_name] = f"{target}()"
            new_root.append(new_obj)
        else:
            new_root.append(obj)

    ET.ElementTree(new_root).write(sys.argv[2], encoding="utf-8", xml_declaration=True)
    print("Transformation successful.")
except Exception as e:
    print(f"Error: {e}")
