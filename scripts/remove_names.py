import glob
import re

for filepath in glob.glob("artifacts/round-14/modular/*.xml"):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    
    # Remove name="Unknown" and any trailing space before it
    new_content = re.sub(r'\s+name="Unknown"', '', content)
    
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"Processed {filepath}")
