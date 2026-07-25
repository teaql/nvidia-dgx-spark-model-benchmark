import glob
import re

for filepath in glob.glob("artifacts/round-14/modular/*.xml"):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    
    # Remove id="0" or id="0u64"
    # Also remove any trailing space before it if possible
    new_content = re.sub(r'\s+id="0(?:u64)?"', '', content)
    
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"Processed {filepath}")
