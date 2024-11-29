# Base image for Rust and dependency caching
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Dependency planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage: compile backend and frontend
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli --version 0.2.93

# Copy the entire project and build backend and frontend
COPY . .
RUN cargo build --release --bin lemorage-portfolio && \
    cd wasm-frontend && \
    cargo build --target wasm32-unknown-unknown --release && \
    wasm-bindgen --out-dir ../static/frontend --target web target/wasm32-unknown-unknown/release/wasm_frontend.wasm

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/lemorage-portfolio /usr/local/bin
COPY --from=builder /app/static /app/static
ENTRYPOINT ["/usr/local/bin/lemorage-portfolio"]
