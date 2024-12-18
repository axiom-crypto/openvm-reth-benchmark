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
MODE=execute # can be execute, prove, or prove-e2e
RUSTFLAGS="-Ctarget-cpu=native" RUST_BACKTRACE=1 OUTPUT_PATH="metrics.json" cargo run --bin openvm-reth-benchmark --release -- --$MODE --block-number 18884864 --rpc-url $RPC_1 --cache-dir rpc-cache
