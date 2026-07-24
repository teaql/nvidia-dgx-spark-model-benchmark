import sys
text = open(sys.argv[1]).read()

# Replace all & with &amp; if they are not already escaped
import re
text = re.sub(r'&(?!(?:amp|lt|gt|quot|apos);)', '&amp;', text)

# Find the last </object>
idx = text.rfind('</object>')
if idx != -1:
    text = text[:idx + len('</object>')] + '\n</root>\n'
else:
    print("Could not find </object>")
    sys.exit(1)

open(sys.argv[2], 'w').write(text)
