# k8s-random-string/Dockerfile
# Multi-stage build: compile Rust binary then run in minimal image.
# Exists to produce a minimal container for k8s-random-string.
# RELEVANT FILES: Cargo.toml, src/main.rs

# -----------------------------------------------------------------------------
# stage 1: build release binary (Alpine/musl)
# -----------------------------------------------------------------------------
FROM rust:1-alpine AS builder

# musl-dev required for linking when targeting x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# copy manifest and lockfile first for better layer caching
COPY Cargo.toml Cargo.lock ./

# dummy main to cache dependencies (real src copied later)
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && rm -rf src

COPY src ./src

# overwrite dummy and build real binary (deps already cached)
RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

# -----------------------------------------------------------------------------
# stage 2: minimal runtime (Alpine)
# -----------------------------------------------------------------------------
FROM alpine:3.19

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/k8s-random-string /app/k8s-random-string

# run as non-root
RUN adduser -D -u 1000 app && chown -R app:app /app
USER app

ENTRYPOINT ["/app/k8s-random-string"]
