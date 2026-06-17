FROM rust:1.96 as builder
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin kv_store_server

FROM debian:trixie-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/kv_store_server .
CMD ["./kv_store_server"]
