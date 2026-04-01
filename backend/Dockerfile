# syntax=docker/dockerfile:1

# -- Stage 1: builder
FROM rust:1.92-bookworm AS builder

WORKDIR /app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Build real source
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# -- Stage 2: runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend ./backend
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./backend"]
