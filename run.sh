#!/bin/bash

set -euo pipefail

MODE="execute-metered" # can be execute-host, execute, execute-metered, prove-app, prove-stark, or prove-evm (needs "evm-verify" feature)
BLOCK_NUMBER=23800838
USE_CUDA=false

usage() {
    cat <<'EOF'
Usage: ./run.sh [cuda] [--mode MODE] [--block-number BLOCK]

Options:
  cuda                 Enable CUDA features for the build and benchmark run.
  --mode MODE          Override the benchmark mode (default: execute-metered).
  --block-number BLOCK Override the block number to execute (default: 23800838).
  -h, --help           Show this message.
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        cuda)
            USE_CUDA=true
            shift
            ;;
        --mode)
            if [[ -z "${2:-}" ]]; then
                echo "Error: --mode requires a value" >&2
                exit 1
            fi
            MODE="$2"
            shift 2
            ;;
        --block-number)
            if [[ -z "${2:-}" ]]; then
                echo "Error: --block-number requires a value" >&2
                exit 1
            fi
            BLOCK_NUMBER="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

mkdir -p rpc-cache
source .env
cd bin/client-eth
cargo openvm build
mkdir -p ../host/elf
SRC="target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth"
DEST="../host/elf/openvm-client-eth"

if [ ! -f "$DEST" ] || ! cmp -s "$SRC" "$DEST"; then
    cp "$SRC" "$DEST"
fi
cd ../..

PROFILE="release"
FEATURES="metrics,jemalloc,aot,unprotected"
# switch to +nightly-2025-08-19 if using tco
TOOLCHAIN="+nightly-2025-08-19" # "+stable"
BIN_NAME="openvm-reth-benchmark-bin"
MAX_SEGMENT_LENGTH=$((1 << 22))
SEGMENT_MAX_CELLS=1200000000
VPMM_PAGE_SIZE=$((4 << 20))
VPMM_PAGES=$((12 * $MAX_SEGMENT_LENGTH/ $VPMM_PAGE_SIZE))

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

RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics.json" VPMM_PAGES=$VPMM_PAGES VPMM_PAGE_SIZE=$VPMM_PAGE_SIZE ./target/$TARGET_DIR/$BIN_NAME \
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
--num-children-internal 3
