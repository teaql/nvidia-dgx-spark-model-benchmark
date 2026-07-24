import urllib.request
import json
import time
import os

url = "http://222.128.77.232:30703/v1/chat/completions"
task_desc = open("benchmarks/round-7/moving-company-operations-30-object-task.md").read()

payload = {
    "model": "/home/dgx007/models/qwen3.6-35b-a3b",
    "messages": [
        {"role": "system", "content": "You are a TeaQL KSML modeling expert. Generate valid KSML XML output ONLY. Do not use markdown blocks. Escape all special characters like & as &amp;."},
        {"role": "user", "content": task_desc}
    ],
    "max_tokens": 8000,
    "temperature": 0.0
}

req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
try:
    print(f"Requesting model {payload['model']}...")
    start = time.time()
    with urllib.request.urlopen(req, timeout=300) as response:
        result = json.loads(response.read().decode("utf-8"))
        content = result["choices"][0]["message"]["content"]
        
        # In case the model returns None, handle it
        if content is None:
            print("Error: The model returned None content.")
            content = "<!-- Model returned None -->"
            
        os.makedirs("artifacts/round-7", exist_ok=True)
        with open("artifacts/round-7/model.xml", "w") as f:
            f.write(content)
        print(f"Generated KSML saved to artifacts/round-7/model.xml in {time.time()-start:.2f}s")
except urllib.error.HTTPError as e:
    print(f"HTTP Error: {e.code} - {e.read().decode('utf-8')}")
except Exception as e:
    print(f"Error: {e}")
