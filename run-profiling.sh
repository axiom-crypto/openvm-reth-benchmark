#!/bin/bash
set -e
cd bin/client-eth
cargo openvm build --no-transpile
mkdir -p ../host/elf
SRC="target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth"
DEST="../host/elf/openvm-client-eth"

if [ ! -f "$DEST" ] || ! cmp -s "$SRC" "$DEST"; then
    cp "$SRC" "$DEST"
fi
cd ../..

mkdir -p rpc-cache
source .env
MODE=prove-e2e # can be execute, tracegen, prove, or prove-e2e
PROFILE="profiling"
FEATURES="bench-metrics,nightly-features,jemalloc"
BLOCK_NUMBER=21882667

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
RUSTFLAGS=$RUSTFLAGS cargo build --bin openvm-reth-benchmark --profile=$PROFILE --no-default-features --features=$FEATURES
PARAMS_DIR="params"
OUTPUT_PATH="metrics.json" samply record --no-open -- ./target/$PROFILE/openvm-reth-benchmark --kzg-params-dir $PARAMS_DIR --$MODE --block-number $BLOCK_NUMBER --rpc-url $RPC_1 --cache-dir rpc-cache