#!/bin/bash

set -e

go fmt && go build

iterations=10
total_time=0

for ((i=1; i<=iterations; i++))
do
    run_time=$(./unzip)
    total_time=$(echo "$total_time + $run_time" | bc)
done

average_time=$(echo "$total_time / $iterations " | bc -l)
echo "Average Elapsed Time: $average_time ms"
