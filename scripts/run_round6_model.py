import urllib.request
import json
import time

url = "http://222.128.77.232:30703/v1/chat/completions"
task_desc = open("benchmarks/round-6/moving-company-operations-30-object-task.md").read()

payload = {
    "model": "nemotron-3-super",
    "messages": [
        {"role": "system", "content": "You are a TeaQL KSML modeling expert. Generate valid KSML XML output ONLY. Do not use markdown blocks. Escape all special characters like & as &amp;."},
        {"role": "user", "content": task_desc}
    ],
    "max_tokens": 4000,
    "temperature": 0.0
}

req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
try:
    print("Requesting model qwen3.6-35b-a3b...")
    start = time.time()
    with urllib.request.urlopen(req, timeout=300) as response:
        result = json.loads(response.read().decode("utf-8"))
        content = result["choices"][0]["message"]["content"]
        import os
        os.makedirs("artifacts/round-6", exist_ok=True)
        with open("artifacts/round-6/model.xml", "w") as f:
            f.write(content)
        print(f"Generated KSML saved to artifacts/round-6/model.xml in {time.time()-start:.2f}s")
except Exception as e:
    print(f"Error: {e}")
