FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
ARG DOMAIN

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ENV DOMAIN=${DOMAIN}
RUN cargo build --release --bin memory_server

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
CMD mkdir /usr/local/bin/data
COPY --from=builder /app/target/release/memory_server /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/memory_server"]
