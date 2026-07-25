#!/bin/bash
set -e

ROUNDS=(



    "27 160"
    "28 180"
    "29 200"
)

for param in "${ROUNDS[@]}"; do
    read -r round scale <<< "$param"
    echo "=========================================="
    echo "STARTING ROUND $round (Scale: $scale)"
    echo "=========================================="
    python3 scripts/run_round.py --round $round --scale $scale
done

echo "ALL 15 ROUNDS COMPLETED SUCCESSFULLY!"
