#!/bin/bash

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
RUN_SCRIPT="$SCRIPT_DIR/run.sh"
CUDA_ARGS=()

usage() {
    cat <<'EOF'
Usage: ./run-benchmarks.sh [--cuda] MODE BLOCK [MODE BLOCK ...]

Runs the benchmark multiple times by invoking run.sh with the specified
mode and block number pairs.

Options:
  --cuda       Enable CUDA for every invocation.
  -h, --help   Show this message.

Example:
  ./run-benchmarks.sh execute-metered 23800838 prove-evm 23850000
  ./run-benchmarks.sh --cuda execute-metered 23800838
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --cuda)
            CUDA_ARGS=("cuda")
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            break
            ;;
    esac
done

if [[ $# -lt 2 || $(( $# % 2 )) -ne 0 ]]; then
    usage >&2
    exit 1
fi

if [[ ! -x "$RUN_SCRIPT" ]]; then
    echo "Error: run.sh not found or not executable at $RUN_SCRIPT" >&2
    exit 1
fi

pair_index=1
while [[ $# -gt 0 ]]; do
    MODE="$1"
    BLOCK_NUMBER="$2"
    shift 2

    echo "[$pair_index] Running benchmark with mode=$MODE block=$BLOCK_NUMBER"
    "$RUN_SCRIPT" "${CUDA_ARGS[@]}" --mode "$MODE" --block-number "$BLOCK_NUMBER"
    echo "[$pair_index] Completed"
    ((pair_index++))
done

