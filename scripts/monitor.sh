#!/bin/bash
LOG_FILE="artifacts/system_metrics.log"
# 确保文件所在的目录存在
mkdir -p artifacts

echo "Starting System Monitor... Logging to $LOG_FILE"

while true; do
    echo "================ $(date) ================" >> $LOG_FILE
    
    echo "--- RAM ---" >> $LOG_FILE
    free -m >> $LOG_FILE
    
    echo "--- CPU ---" >> $LOG_FILE
    # 获取 top 的前 5 行（包含 Load Average 和 CPU 总体百分比）
    top -b -n 1 | head -n 5 >> $LOG_FILE
    
    echo "--- GPU ---" >> $LOG_FILE
    # 提取 GPU 利用率与显存情况（如果存在 nvidia-smi）
    if command -v nvidia-smi &> /dev/null; then
        nvidia-smi --query-gpu=index,name,utilization.gpu,memory.used,memory.total --format=csv,noheader >> $LOG_FILE
    else
        echo "No NVIDIA GPU detected." >> $LOG_FILE
    fi
    
    sleep 5
done
