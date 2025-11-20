#!/bin/bash
# Usage:
# ./generate_inputs.sh 22000000                    # Single block
# ./generate_inputs.sh 22000000,22000001,22000002  # Multiple blocks (comma-separated)
# ./generate_inputs.sh 1000-2000                   # Range of blocks (1000 to 2000)

BLOCK_NUMBERS=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    *)
      if [[ -z "$BLOCK_NUMBERS" ]]; then
        BLOCK_NUMBERS="$1"
      fi
      shift
      ;;
  esac
done

if [[ -z "$BLOCK_NUMBERS" ]]; then
  echo "❌ No block numbers provided"
  exit 1
fi

# Function to expand ranges (e.g., "1000-1005" -> "1000 1001 1002 1003 1004 1005")
expand_ranges() {
  local input="$1"
  local result=()

  IFS=',' read -ra PARTS <<< "$input"
  for part in "${PARTS[@]}"; do
    if [[ "$part" =~ ^([0-9]+)-([0-9]+)$ ]]; then
      # It's a range
      local start="${BASH_REMATCH[1]}"
      local end="${BASH_REMATCH[2]}"
      for i in $(seq "$start" "$end"); do
        result+=("$i")
      done
    elif [[ "$part" =~ ^[0-9]+$ ]]; then
      # It's a single number
      result+=("$part")
    else
      echo "ERROR: Invalid block specification: $part" >&2
      exit 1
    fi
  done

  echo "${result[@]}"
}

# Expand block ranges
EXPANDED_BLOCKS=($(expand_ranges "$BLOCK_NUMBERS"))

INPUTS_DIR="$(pwd)/inputs"
RPC_CACHE="$(pwd)/rpc-cache"

cd openvm-reth-benchmark/bin/host
# The binary has a static include_bytes! but doesn't really need this ELF for proof input generation:
mkdir -p elf
touch elf/openvm-client-eth
cargo build --bin openvm-reth-benchmark-bin --release
cd ../..

RETH_BIN="$(pwd)/target/release/openvm-reth-benchmark-bin"
# cur dir is openvm-reth-benchmark

INPUTS_DIR="$(pwd)/inputs"
RPC_CACHE="$(pwd)/rpc-cache"
mkdir -p $INPUTS_DIR
mkdir -p $RPC_CACHE

for block in "${EXPANDED_BLOCKS[@]}"; do
    echo "⚙️  Generating reth input for block $block"
    $RETH_BIN --block-number $block --mode make-input --rpc-url $RPC_URL_1 --cache-dir $RPC_CACHE --generated-input-path "$INPUTS_DIR/${block}.json"
    echo "✅ Generated input for block $block"
done
