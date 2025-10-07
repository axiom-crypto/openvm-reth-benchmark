FROM rust:1.86-slim-bookworm AS builder

# System build deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    cmake \
    clang \
    libclang-dev \
    curl \
    git \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Toolchains: stable for cargo-openvm, nightly for tco build
RUN rustup toolchain install nightly-2025-08-19 \
  && rustup component add rust-src --toolchain nightly-2025-08-19

# Install cargo-openvm (builds the guest ELF)
RUN cargo +1.86 install --git https://github.com/openvm-org/openvm.git --locked --force cargo-openvm

WORKDIR /app
# Copy only Rust workspace files to keep build cache stable when server/ changes
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates/ ./crates/
COPY bin/ ./bin/
COPY rustfmt.toml ./

# Build guest ELF and place where host expects it
WORKDIR /app/bin/client-eth
RUN cargo openvm build --no-transpile --profile=release \
  && mkdir -p ../host/elf \
  && cp target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth ../host/elf/

# Build host binary
WORKDIR /app
ENV JEMALLOC_SYS_WITH_MALLOC_CONF="retain:true,background_thread:true,metadata_thp:always,dirty_decay_ms:10000,muzzy_decay_ms:10000,abort_conf:true"
ARG FEATURES="metrics,jemalloc,tco,unprotected,nightly-features"
ARG PROFILE="release"
ENV RUSTFLAGS="-Ctarget-cpu=native"
RUN cargo +nightly-2025-08-19 build --bin openvm-reth-benchmark-bin --profile=${PROFILE} --no-default-features --features=${FEATURES}

# Runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates python3 python3-venv \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/openvm-reth-benchmark-bin /usr/local/bin/openvm-reth-benchmark-bin
COPY --from=builder /app/bin/host/elf/openvm-client-eth /app/bin/host/elf/openvm-client-eth
COPY server /app/server

RUN python3 -m venv /opt/venv \
  && . /opt/venv/bin/activate \
  && pip install --no-cache-dir -r /app/server/requirements.txt

ENV RUST_LOG="info,p3_=warn" \
    OUTPUT_PATH="metrics.json" \
    JEMALLOC_SYS_WITH_MALLOC_CONF="retain:true,background_thread:true,metadata_thp:always,dirty_decay_ms:10000,muzzy_decay_ms:10000,abort_conf:true" \
    KZG_PARAMS_DIR="/root/.openvm/params"

# Useful mounts for cache/params
VOLUME ["/app/rpc-cache", "/root/.openvm/params"]

# The binary takes the same CLI flags as run.sh uses at the end
# Example:
#   docker run --rm -e RPC_1=... -v $(pwd)/rpc-cache:/app/rpc-cache \
#     openvm-reth-bench \
#     --kzg-params-dir /root/.openvm/params \
#     --mode execute --block-number 23100006 \
#     --rpc-url $RPC_1 --cache-dir /app/rpc-cache \
#     --app-log-blowup 1 --leaf-log-blowup 1 \
#     --internal-log-blowup 2 --root-log-blowup 3 \
#     --max-segment-length 4194204 --segment-max-cells 700000000 \
#     --num-children-leaf 1 --num-children-internal 3

ENV PATH="/opt/venv/bin:${PATH}" \
    OVM_BIN="/usr/local/bin/openvm-reth-benchmark-bin"

EXPOSE 8000
ENTRYPOINT ["uvicorn", "server.main:app", "--host", "0.0.0.0", "--port", "8000"]


