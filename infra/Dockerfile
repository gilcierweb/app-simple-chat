# ─── Stage 1: builder ─────────────────────────────────────────────────────────
FROM rust:1.82-bookworm AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build the real source
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# ─── Stage 2: runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./server"]
