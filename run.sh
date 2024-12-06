#!/bin/bash
set -e
cd bin/client-eth
cargo axiom build
# AXIOM_BUILD_DEBUG=1 cargo axiom build
mkdir -p ../host/elf
cp target/riscv32im-risc0-zkvm-elf/release/rsp-client-eth ../host/elf/
# cp target/riscv32im-risc0-zkvm-elf/debug/rsp-client-eth ../host/elf/
cd ../..

mkdir -p rpc-cache
source .env
RUSTFLAGS="-Ctarget-cpu=native" RUST_BACKTRACE=1 cargo run --bin rsp --release -- --block-number 18884864 --rpc-url $RPC_1 --cache-dir rpc-cache --prove
