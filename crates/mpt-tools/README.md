# MPT Development Tools

This crate provides development tools for profiling and benchmarking Merkle Patricia Trie (MPT) operations. These tools are designed to help optimize the performance of MPT operations used in the main zkvm benchmark.

**Note:** All commands should be run from the `crates/mpt-tools/` directory.

## Tools

### Data Generation

```bash
cargo run --bin generate_benchmark_data                    # Default block 21000000
BLOCK=18884864 cargo run --bin generate_benchmark_data     # Custom block
```

### Memory Profiling

```bash
cargo run --bin mpt_profiler --features dhat-heap          # Profile all operations
cargo run --bin mpt_profiler --features dhat-heap update   # Profile specific operation
BLOCK=18884864 cargo run --bin mpt_profiler --features dhat-heap update
```

### Performance Benchmarking

```bash
cargo bench                                                 # Default block 21000000
BLOCK=18884864 cargo bench                                  # Custom block
```

## Workflow

```bash
cargo run --bin generate_benchmark_data                     # Generate data (21000000.bin)
cargo run --bin mpt_profiler --features dhat-heap           # Profile memory
cargo bench                                                 # Benchmark performance
```

## Requirements

- `RPC_1` environment variable (Ethereum RPC endpoint)
- View `dhat-heap.json` files at https://nnethercote.github.io/dh_view/dh_view.html 