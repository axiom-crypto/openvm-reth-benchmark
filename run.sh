#!/bin/bash

USE_CUDA=true
# if [ "$1" == "cuda" ]; then
#     USE_CUDA=true
# fi

cd bin/client-eth
cargo openvm build
mkdir -p ../host/elf
SRC="target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth"
DEST="../host/elf/openvm-client-eth"

if [ ! -f "$DEST" ] || ! cmp -s "$SRC" "$DEST"; then
    cp "$SRC" "$DEST"
fi
cd ../..

mkdir -p rpc-cache
source .env
MODE=prove-stark # can be execute, execute-metered, prove-app, prove-stark, or prove-evm (needs "evm-verify" feature)
PROFILE="release"
FEATURES="metrics,jemalloc,tco,unprotected"
# switch to +nightly-2025-08-19 if using tco
TOOLCHAIN="+nightly-2025-08-19" # "+stable"
BIN_NAME="openvm-reth-benchmark-bin"
MAX_SEGMENT_LENGTH=8378608 # 2^23 - 10000
SEGMENT_MAX_CELLS=1000000000 # 1B

if [ "$USE_CUDA" = "true" ]; then
    FEATURES="$FEATURES,cuda"
else
    FEATURES="$FEATURES,nightly-features"
fi
if [ "$MODE" = "prove-evm" ]; then
    FEATURES="$FEATURES,evm-verify"
fi

arch=$(uname -m)
case $arch in
arm64|aarch64)
    RUSTFLAGS="-Ctarget-cpu=native"
    ;;
x86_64|amd64)
    RUSTFLAGS="-Ctarget-cpu=native"
    ;;
*)
echo "Unsupported architecture: $arch"
exit 1
;;
esac
export JEMALLOC_SYS_WITH_MALLOC_CONF="retain:true,background_thread:true,metadata_thp:always,dirty_decay_ms:10000,muzzy_decay_ms:10000,abort_conf:true"
RUSTFLAGS=$RUSTFLAGS cargo $TOOLCHAIN build --bin $BIN_NAME --profile=$PROFILE --no-default-features --features=$FEATURES
PARAMS_DIR="$HOME/.openvm/params/"

# Use target/debug if profile is dev
if [ "$PROFILE" = "dev" ]; then
    TARGET_DIR="debug"
else
    TARGET_DIR="$PROFILE"
fi

export CUDA_DEVICE_ORDER=PCI_BUS_ID
DEVICE_ID=1
BASE_BLOCK=22000000
TOTAL_BLOCKS=1000
BLOCKS_PER_DEVICE=$((TOTAL_BLOCKS / 8))

mkdir -p metrics
mkdir -p logs

# Create failure tracking files
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
FAILURE_LOG="logs/failed-blocks-$TIMESTAMP.log"
SUCCESS_LOG="logs/successful-blocks-$TIMESTAMP.log"

# Initialize logs
echo "Failed blocks log started at $(date)" > "$FAILURE_LOG"
echo "Successful blocks log started at $(date)" > "$SUCCESS_LOG"

# Function to process blocks for a specific device
process_device() {
    local DEVICE_ID=$1
    local START_BLOCK=$((BASE_BLOCK + DEVICE_ID * BLOCKS_PER_DEVICE))
    local END_BLOCK=$((BASE_BLOCK + (DEVICE_ID + 1) * BLOCKS_PER_DEVICE))

    echo "Device $DEVICE_ID: Processing blocks $START_BLOCK to $((END_BLOCK - 1))"

    for BLOCK_NUMBER in $(seq $START_BLOCK $((END_BLOCK - 1))); do
        echo "Device $DEVICE_ID: Processing block $BLOCK_NUMBER"

        CUDA_VISIBLE_DEVICES=$DEVICE_ID RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics/metrics-${BLOCK_NUMBER}.json" ./target/$TARGET_DIR/$BIN_NAME \
        --kzg-params-dir $PARAMS_DIR \
        --mode $MODE \
        --block-number $BLOCK_NUMBER \
        --rpc-url $RPC_1 \
        --cache-dir rpc-cache \
        --app-log-blowup 1 \
        --leaf-log-blowup 1 \
        --internal-log-blowup 2 \
        --root-log-blowup 3 \
        --max-segment-length $MAX_SEGMENT_LENGTH \
        --segment-max-cells $SEGMENT_MAX_CELLS \
        --num-children-leaf 1 \
        --num-children-internal 3 2>&1 | tee logs/log-${BLOCK_NUMBER}.txt

        EXIT_CODE=${PIPESTATUS[0]}

        if [ $EXIT_CODE -eq 0 ]; then
            echo "Device $DEVICE_ID: Successfully completed block $BLOCK_NUMBER"
            echo "[$(date '+%Y-%m-%d %H:%M:%S')] Device $DEVICE_ID: Block $BLOCK_NUMBER - SUCCESS" >> "$SUCCESS_LOG"
        else
            echo "Device $DEVICE_ID: FAILED block $BLOCK_NUMBER with exit code $EXIT_CODE"
            echo "[$(date '+%Y-%m-%d %H:%M:%S')] Device $DEVICE_ID: Block $BLOCK_NUMBER failed with exit code $EXIT_CODE" >> "$FAILURE_LOG"
        fi
    done

    echo "Device $DEVICE_ID: Finished all blocks"
}

for DEVICE_ID in {0..7}; do
    process_device $DEVICE_ID &
done

# Wait for all devices to complete
wait
echo "All devices completed processing!"
