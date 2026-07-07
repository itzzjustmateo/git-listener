FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
COPY migrations/ migrations/
COPY config/ config/
COPY sqlx-data.json* ./

ENV SQLX_OFFLINE=true
RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/git-listener /app/git-listener
COPY config/ /app/config/
COPY migrations/ /app/migrations/

ENV GL_APP__ENVIRONMENT=production

EXPOSE 8080 8081

ENTRYPOINT ["/app/git-listener"]
