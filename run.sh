#!/bin/bash

USE_CUDA=false
if [ "$1" == "cuda" ]; then
    USE_CUDA=true
fi

set -e

mkdir -p rpc-cache
source .env
MODE=execute # can be execute, execute-metered, execute-native, prove-app, prove-stark, or prove-evm (needs "evm-verify" feature)

# Skip cargo openvm build for execute-native mode
if [ "$MODE" != "execute-native" ]; then
    cd bin/client-eth
    cargo openvm build
    mkdir -p ../host/elf
    SRC="target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth"
    DEST="../host/elf/openvm-client-eth"

    if [ ! -f "$DEST" ] || ! cmp -s "$SRC" "$DEST"; then
        cp "$SRC" "$DEST"
    fi
    cd ../..
fi

PROFILE="profiling"
FEATURES="metrics,jemalloc,tco,unprotected"
BLOCK_NUMBER=23100006
# switch to +nightly-2025-08-19 if using tco
TOOLCHAIN="+nightly-2025-08-19" # "+stable"
BIN_NAME="openvm-reth-benchmark-bin"
MAX_SEGMENT_LENGTH=4194204
SEGMENT_MAX_CELLS=700000000
VPMM_PAGE_SIZE=$((4<<20))
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
