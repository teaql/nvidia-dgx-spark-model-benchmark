import re
import sys

def fix_xml(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Fix type keyword errors: replace type=" with record_type="
    content = re.sub(r'\btype="', 'record_type="', content)

    # Fix KSML-PRIVACY-001-ERR for access_token.token_id
    content = content.replace('<access_token ', '<access_token _audit_mask_fields="token_id" ')

    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    print("Fixed keywords and privacy tags in XML.")

fix_xml(sys.argv[1])
