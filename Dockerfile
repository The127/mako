FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY api_server/Cargo.toml api_server/
COPY api_client/Cargo.toml api_client/
COPY cli/Cargo.toml cli/
COPY shared/Cargo.toml shared/

# Cache dependencies
RUN mkdir -p api_server/src api_client/src cli/src shared/src && \
    echo "fn main() {}" > api_server/src/main.rs && \
    echo "" > api_client/src/lib.rs && \
    echo "fn main() {}" > cli/src/main.rs && \
    echo "" > shared/src/lib.rs && \
    cargo build --release --bin mako && \
    rm -rf api_server/src api_client/src cli/src shared/src

COPY api_server/ api_server/
COPY api_client/ api_client/
COPY cli/ cli/
COPY shared/ shared/

RUN touch api_server/src/main.rs && cargo build --release --bin mako


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mako /usr/local/bin/mako

EXPOSE 8080

ENTRYPOINT ["mako"]
