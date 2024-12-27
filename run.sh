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
PROFILE="release"

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
RUSTFLAGS=$RUSTFLAGS cargo build --bin openvm-reth-benchmark --profile=$PROFILE --no-default-features --features=$FEATURES
RUST_BACKTRACE=1 OUTPUT_PATH="metrics.json" ./target/$PROFILE/openvm-reth-benchmark --$MODE --block-number 18884864 --rpc-url $RPC_1 --cache-dir rpc-cache
