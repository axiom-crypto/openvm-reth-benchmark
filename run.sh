#!/bin/bash
set -e
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
MODE=execute # can be execute, execute-metered, prove-app, prove-stark, or prove-evm
PROFILE="release"
FEATURES="metrics,nightly-features,jemalloc,evm-verify"
BLOCK_NUMBER=21000000

arch=$(uname -m)
case $arch in
arm64|aarch64)
    RUSTFLAGS="-Ctarget-cpu=native"
    ;;
x86_64|amd64)
    RUSTFLAGS="-Ctarget-cpu=native -C target-feature=+avx512f"
    ;;
*)
echo "Unsupported architecture: $arch"
exit 1
;;
esac
export JEMALLOC_SYS_WITH_MALLOC_CONF="retain:true,background_thread:true,metadata_thp:always,dirty_decay_ms:-1,muzzy_decay_ms:-1,abort_conf:true"
RUSTFLAGS=$RUSTFLAGS cargo build --bin openvm-reth-benchmark-bin --profile=$PROFILE --no-default-features --features=$FEATURES
PARAMS_DIR="params"

# Use target/debug if profile is dev
if [ "$PROFILE" = "dev" ]; then
    TARGET_DIR="debug"
else
    TARGET_DIR="$PROFILE"
fi

RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics.json" ./target/$TARGET_DIR/openvm-reth-benchmark-bin --kzg-params-dir $PARAMS_DIR --mode $MODE --block-number $BLOCK_NUMBER --rpc-url $RPC_1 --cache-dir rpc-cache
