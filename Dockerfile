# syntax=docker/dockerfile:1
# ------------------------------------------------------------------------------
# Stage 1: Build static release binary with Rust and musl
# ------------------------------------------------------------------------------
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev gcc make

WORKDIR /build

# Cache dependency builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "pub fn version() {}" > src/lib.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy full source and assets
COPY src ./src
COPY assets ./assets
RUN touch src/main.rs src/lib.rs
RUN cargo build --release

# ------------------------------------------------------------------------------
# Stage 2: Minimal runtime image
# ------------------------------------------------------------------------------
FROM alpine:3.20 AS runtime

RUN addgroup -g 10001 -S ayvu && \
    adduser -u 10001 -S ayvu -G ayvu

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/ayvu /usr/local/bin/ayvu
COPY --from=builder /build/assets /app/assets

RUN chown -R ayvu:ayvu /app

EXPOSE 3100

HEALTHCHECK --interval=30s --timeout=5s --start-period=3s --retries=3 \
  CMD wget -q -O /dev/null http://127.0.0.1:3100/health || exit 1

USER 10001:10001

ENTRYPOINT ["/usr/local/bin/ayvu"]
CMD ["serve", "--host", "0.0.0.0", "--port", "3100"]
