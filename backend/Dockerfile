# syntax=docker/dockerfile:1

# -- Stage 1: builder
FROM rust:1-bookworm AS builder

# Install system dependencies for diesel/postgresql
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# dummy build to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# build real source
COPY src ./src
COPY migrations ./migrations
COPY locales ./locales
RUN cargo build --release

# -- Stage 2: runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and required assets
COPY --from=builder /app/target/release/backend ./backend
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/locales ./locales

# Create non-root user for security
RUN useradd -ms /bin/bash appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=8080

CMD ["./backend"]
