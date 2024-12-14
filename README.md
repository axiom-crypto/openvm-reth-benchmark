# Reth Succinct Processor (RSP)

A minimal implementation of generating zero-knowledge proofs of EVM block execution using [Reth](https://github.com/paradigmxyz/reth). Supports both Ethereum and OP Stack.

> [!CAUTION]
>
> This repository is still an active work-in-progress and is not audited or meant for production usage.

## [WIP] openvm

Instructions to run:
In openvm repo:

```bash
cd crates/cargo-axiom
cargo install --force --path .
```

to install `cargo axiom`.

In this repo,

```bash
cd bin/client-eth
cargo axiom build
mkdir -p ../host/elf
cp target/riscv32im-risc0-zkvm-elf/release/rsp-client-eth ../host/elf/
cd ../..
```

install any rust stuff it tells you to.

To run

```bash
mkdir rpc-cache
MODE=prove-e2e # can be execute, prove, or prove-e2e
RUSTFLAGS="-Ctarget-cpu=native" RUST_LOG=info OUTPUT_PATH="metrics.json" cargo run --bin rsp --release -- --$MODE --block-number 20526624 --rpc-url $RPC_1 --cache-dir rpc-cache
```

Get an archive node rpc url for $RPC_1 for eth mainnet.
Optional flag: `--collect-metrics` will collect metrics for flamegraphs [NOTE: this slows down execution significantly].
In order to run `prove-e2e`, you need to download the KZG trusted setup for halo2 into the `params` folder.

```
bash path-to-afs-prototype/extensions/native/recursion/trusted_setup_s3.sh
```

This will download the trusted setup from s3 and put it in a `params` folder. Set `PARAMS_DIR` to the path of the `params` folder.

To collect detailed metrics of instructions used and trace cell breakdowns, run

```bash
RUSTFLAGS="-Ctarget-cpu=native" RUST_LOG=info OUTPUT_PATH="metrics.json" cargo run --bin rsp --profile=profiling -- --execute --block-number 20526624 --rpc-url $RPC_1 --cache-dir rpc-cache --collect-metrics
```

This will only execute the runtime (without proving), but collect many metrics that are output into the `OUTPUT_PATH` file.

```bash
<afs-prototype>/ci/scripts/metric_unify/flamegraph.py $OUTPUT_PATH # generates flamegraphs
<afs-prototype>/ci/scripts/metric_unify/main.py $OUTPUT_PATH --aggregation-json <afs-prototype>/ci/scripts/metric_unify/aggregation.json # generates markdown
```

TODO: upload the specialized script to generating pretty markdown.

## Getting Started

To use RSP, you must first have [Rust](https://www.rust-lang.org/tools/install) installed and openvm installed to build the client programs. Then follow the instructions below.

### Installing the CLI

In the root directory of this repository, run:

```console
cargo install --locked --path bin/host
```

and the command `rsp` will be installed.

### RPC Node Requirement

RSP fetches block and state data from a JSON-RPC node. You must use an archive node which preserves historical intermediate trie nodes needed for fetching storage proofs.

In Geth, the archive mode can be enabled with the `--gcmode=archive` option. You can also use an RPC provider that offers archive data access.

> [!TIP]
>
> Don't have access to such a node but still want to try out RSP? Use [`rsp-tests`](https://github.com/succinctlabs/rsp-tests) to get quickly set up with an offline cache built for selected blocks.

### Running the CLI

The host CLI automatically identifies the underlying chain type using the RPC (with the `eth_chainId` call). Simply suppply a block number and an RPC URL:

```console
rsp --block-number 18884864 --rpc-url <RPC>
```

which outputs logs similar to:

```log
2024-07-15T00:49:03.857638Z  INFO rsp_host_executor: fetching the current block and the previous block
2024-07-15T00:49:04.547738Z  INFO rsp_host_executor: setting up the spec for the block executor
2024-07-15T00:49:04.551198Z  INFO rsp_host_executor: setting up the database for the block executor
2024-07-15T00:49:04.551268Z  INFO rsp_host_executor: executing the block and with rpc db: block_number=18884864, transaction_count=30
2024-07-15T00:50:51.526624Z  INFO rsp_host_executor: verifying the state root
...
```

The host CLI executes the block while fetching additional data necessary for offline execution. The same execution and verification logic is then run inside the zkVM. No actual proof is generated from this command, but it will print out a detailed execution report and statistics on the # of cycles to a CSV file (can be specified by the `--report-path` argument).

You can also run the CLI directly by running the following command:

```bash
cargo run --bin rsp --release -- --block-number 18884864 --rpc-url <RPC>
```

or by providing the RPC URL in the `.env` file (or otherwise setting the relevant env vars) and specifying the chain id in the CLI command like this:

```bash
cargo run --bin rsp --release -- --block-number 18884864 --chain-id <chain-id>
```

#### Using cached client input

The client input (witness) generated by executing against RPC can be cached to speed up iteration of the client program by supplying the `--cache-dir` option:

```bash
cargo run --bin rsp --release -- --block-number 18884864 --chain-id <chain-id> --cache-dir /path/to/cache
```

Note that even when utilizing a cached input, the host still needs access to the chain ID to identify the network type, either through `--rpc-url` or `--chain-id`. To run the host completely offline, use `--chain-id` for this.

## Running Tests

End-to-end integration tests are available. To run these tests, utilize the `.env` file (see [example](./.env.example)) or manually set these environment variables:

```bash
export RPC_1="YOUR_ETHEREUM_MAINNET_RPC_URL"
export RPC_10="YOUR_OP_MAINNET_RPC_URL"
export RPC_59144="YOUR_LINEA_MAINNET_RPC_URL"
```

Note that these JSON-RPC nodes must fulfill the [RPC node requirement](#rpc-node-requirement).

Then execute:

```bash
RUST_LOG=info cargo test -p rsp-host-executor --release e2e -- --nocapture
```

### Generating Proofs

If you want to actually generate proofs, you can run the CLI using the `--prove` argument, like this:

```bash
cargo run --bin rsp --release -- --block-number 18884864 --chain-id <chain-id> --prove
```

This will generate proofs locally on your machine. Given how large these programs are, it might take a while for the proof to generate.

#### Run with prover network

If you want to run proofs using Succinct's [prover network](https://docs.succinct.xyz/generating-proofs/prover-network.html), follow the sign-up instructions, and run the command with the following environment variables prefixed:

```bash
SP1_PROVER=network SP1_PRIVATE_KEY=
```

To specify a custom prover network RPC, you can use the `PROVER_NETWORK_RPC` environment variable.

#### Run with GPU

To generate proofs locally on a GPU, you can enable the `cuda` feature in the CLI, which will enable it in the SDK. Make sure to read the instructions [here](https://github.com/succinctlabs/sp1/blob/fb967e8c409b318d18985f8f92353e93d38c7cda/book/generating-proofs/hardware-acceleration/cuda.md) to make sure you have all required dependencies installed. You can run it with a command like this:

```bash
cargo run --bin rsp --release --features cuda -- --block-number 18884864 --chain-id <chain-id> --prove
```

## FAQ

### Building the client programs manually

By default, the `build.rs` in the `bin/host` crate will rebuild the client programs every time they are modified. To manually build the client programs, you can run these commands (ake sure you have the [SP1 toolchain](https://docs.succinct.xyz/getting-started/install.html) installed):

```console
cd ./bin/client-eth
cargo prove build --ignore-rust-version
```

To build the Optimism client ELF program:

```console
cd ./bin/client-op
cargo prove build --ignore-rust-version
```

### What are good testing blocks

A good small block to test on for Ethereum mainnet is: `20526624`.
