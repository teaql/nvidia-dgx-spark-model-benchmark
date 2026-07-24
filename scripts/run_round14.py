import urllib.request
import json
import time
import os

url = "http://222.128.77.232:30704/v1/chat/completions"

def run_round(round_num):
    task_desc = open(f"benchmarks/round-{round_num}/task.md").read()
    preserved = open(f"benchmarks/round-{round_num}/preserved-core-objects.xml").read()
    
    payload = {
        "model": "nemotron-3-super",
        "messages": [
            {"role": "system", "content": "You are a TeaQL KSML modeling expert. Generate valid KSML XML output ONLY. The root must contain data_service=\"sqlite\". Ensure no fields are named \"type\". Generate at least 180 unique direct-child objects. Do not use markdown blocks. Escape all special characters like & as &amp;. Preserve the exact platform, merchant, and employee definitions provided."},
            {"role": "user", "content": f"{task_desc}\n\n# Preserved Core Objects:\n```xml\n{preserved}\n```"}
        ],
        "max_tokens": 12000,
        "temperature": 0.0,
        "chat_template_kwargs": {
            "enable_thinking": True,
            "reasoning_budget": 4096,
        },
    }

    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
    try:
        print(f"Requesting Nemotron model for Round {round_num} (180+ objects)...")
        start = time.time()
        with urllib.request.urlopen(req, timeout=2400) as response:
            result = json.loads(response.read().decode("utf-8"))
            content = result["choices"][0]["message"]["content"]
            reasoning = result["choices"][0]["message"].get("reasoning_content", "")
            finish_reason = result["choices"][0].get("finish_reason", "unknown")
            
            if content is None:
                content = "<!-- Model returned None -->"
                
            os.makedirs(f"artifacts/round-{round_num}", exist_ok=True)
            with open(f"artifacts/round-{round_num}/model.xml", "w") as f:
                f.write(content.strip())
            with open(f"artifacts/round-{round_num}/reasoning.txt", "w") as f:
                f.write(reasoning.strip())
            print(f"Generated KSML saved to artifacts/round-{round_num}/model.xml in {time.time()-start:.2f}s (finish_reason: {finish_reason})")
    except Exception as e:
        print(f"Error in Round {round_num}: {e}")

run_round(14)
