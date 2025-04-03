#!/bin/bash

gh run list --workflow reth-benchmark.yml --json displayTitle,databaseId --limit 10 | jq -c '.[]' | while IFS= read -r line; do
	title=$(echo "$line" | jq '.displayTitle')
	id=$(echo "$line" | jq '.databaseId')
	echo "https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/$id"
	echo $(echo $title | awk '{print $NF}' | head -c 8)
	gh run view -R axiom-crypto/openvm-reth-benchmark $id --log | grep Cells | ./log_parse.py
	echo
done