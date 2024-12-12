#!/bin/bash
set -e
cd bin/client-eth
cargo axiom build
# AXIOM_BUILD_DEBUG=1 cargo axiom build
mkdir -p ../host/elf
SRC="target/riscv32im-risc0-zkvm-elf/release/rsp-client-eth"
DEST="../host/elf/rsp-client-eth"

if [ ! -f "$DEST" ] || ! cmp -s "$SRC" "$DEST"; then
    cp "$SRC" "$DEST"
fi
cd ../..

mkdir -p rpc-cache
source .env
MODE=execute # can be execute, prove, or prove-e2e
RUSTFLAGS="-Ctarget-cpu=native" RUST_BACKTRACE=1 OUTPUT_PATH="metrics.json" cargo run --bin rsp --release -- --$MODE --block-number 18884864 --rpc-url $RPC_1 --cache-dir rpc-cache
