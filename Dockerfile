FROM rust:1.96-slim-bookworm as builder

RUN apt update && \
    apt install -y libpq-dev build-essential && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release --all-features

# Build diesel_cli here
RUN cargo install diesel_cli --no-default-features --features postgres --version 2.2.12

FROM debian:bookworm-slim AS runner

# Install ONLY the runtime postgres library (libpq5)
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy your app binary
COPY --from=builder /app/target/release/primeproperty-axum .

# 2. Copy the diesel binary from the builder's cargo bin
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# 3. Copy config/migration files
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml .

EXPOSE 8000
ENTRYPOINT ["./primeproperty-axum"]
