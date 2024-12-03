#!/bin/sh
cd bin/client-eth
cargo axiom build
mkdir -p ../host/elf
cp target/riscv32im-risc0-zkvm-elf/release/rsp-client-eth ../host/elf/
cd ../..

mkdir -p rpc-cache
source .env
RUSTFLAGS="-Ctarget-cpu=native" RUST_LOG=info cargo run --bin rsp --release -- --prove --block-number 20526624 --rpc-url $RPC_1 --cache-dir rpc-cache

