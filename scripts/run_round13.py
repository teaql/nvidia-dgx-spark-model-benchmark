import urllib.request
import json
import time
import os

url = "http://222.128.77.232:30704/v1/chat/completions"

def run_round(round_num, target_obj_count):
    task_desc = open(f"benchmarks/round-{round_num}/task.md").read()
    payload = {
        "model": "nemotron-3-super",
        "messages": [
            {"role": "system", "content": f"You are a TeaQL KSML modeling expert. Generate valid KSML XML output ONLY. The root must contain data_service=\"sqlite\". Ensure no fields are named \"type\". Generate EXACTLY {target_obj_count} objects. Do not use markdown blocks. Escape all special characters like & as &amp;."},
            {"role": "user", "content": task_desc}
        ],
        "max_tokens": 20000,
        "temperature": 0.0,
        "chat_template_kwargs": {
            "enable_thinking": True,
            "reasoning_budget": 2048,
        },
    }

    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"})
    try:
        print(f"Requesting Nemotron model for Round {round_num} ({target_obj_count} objects)...")
        start = time.time()
        with urllib.request.urlopen(req, timeout=1200) as response:
            result = json.loads(response.read().decode("utf-8"))
            content = result["choices"][0]["message"]["content"]
            reasoning = result["choices"][0]["message"].get("reasoning_content", "")
            
            if content is None:
                content = "<!-- Model returned None -->"
                
            os.makedirs(f"artifacts/round-{round_num}", exist_ok=True)
            with open(f"artifacts/round-{round_num}/model.xml", "w") as f:
                f.write(content.strip())
            with open(f"artifacts/round-{round_num}/reasoning.txt", "w") as f:
                f.write(reasoning.strip())
            print(f"Generated KSML saved to artifacts/round-{round_num}/model.xml in {time.time()-start:.2f}s")
    except Exception as e:
        print(f"Error in Round {round_num}: {e}")

run_round(13, 30)
