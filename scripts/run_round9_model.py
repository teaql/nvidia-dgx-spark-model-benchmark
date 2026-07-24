import urllib.request
import json
import time
import os
import sys

url = "http://222.128.77.232:30703/v1/chat/completions"

# Wait for server to be up
print("Waiting for vLLM server to be ready on port 30703...")
while True:
    try:
        urllib.request.urlopen("http://222.128.77.232:30703/v1/models", timeout=5)
        print("Server is up!")
        break
    except Exception:
        time.sleep(5)

task_desc = open("benchmarks/round-9/moving-company-180-object-task.md").read()

payload = {
    "model": "/home/dgx007/models/qwen3.6-35b-a3b",
    "messages": [
        {"role": "system", "content": "You are a TeaQL KSML modeling expert. Generate valid KSML XML output ONLY. The root must contain data_service=\"sqlite\". Ensure no fields are named \"type\". Generate the full model with all 180 objects. Do not use markdown blocks. Escape all special characters like & as &amp;."},
        {"role": "user", "content": task_desc}
    ],
    "max_tokens": 32768,
    "temperature": 0.0
}

req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
try:
    print(f"Requesting model {payload['model']}...")
    start = time.time()
    with urllib.request.urlopen(req, timeout=1200) as response:
        result = json.loads(response.read().decode("utf-8"))
        content = result["choices"][0]["message"]["content"]
        
        # In case the model returns None, handle it
        if content is None:
            print("Error: The model returned None content.")
            content = "<!-- Model returned None -->"
            
        os.makedirs("artifacts/round-9", exist_ok=True)
        with open("artifacts/round-9/model.xml", "w") as f:
            f.write(content.strip())
        print(f"Generated KSML saved to artifacts/round-9/model.xml in {time.time()-start:.2f}s")
        print(f"Finish reason: {result['choices'][0].get('finish_reason', 'unknown')}")
except Exception as e:
    print(f"Error: {e}")
