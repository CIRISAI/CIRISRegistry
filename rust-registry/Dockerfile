# CIRISRegistry Rust - Multi-stage build

# Builder stage
FROM rust:1.93-bookworm AS builder

# Install protobuf compiler
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy proto file to parent dir (build.rs expects ../protocol/)
COPY protocol/ciris_registry.proto /protocol/

# Copy Cargo files for dependency caching
COPY rust-registry/Cargo.toml rust-registry/Cargo.lock* ./
COPY rust-registry/build.rs ./

# Create dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies only (may fail without full source, that's ok)
RUN cargo build --release || true

# Copy actual source code
COPY rust-registry/src ./src
COPY rust-registry/migrations ./migrations

# Build the actual application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/ciris-registry /app/ciris-registry

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Create non-root user
RUN useradd -r -s /bin/false ciris
USER ciris

# Environment defaults
ENV GRPC_PORT=50051
ENV HTTP_PORT=8080
ENV LOG_LEVEL=info
ENV RUST_LOG=info

EXPOSE 50051
EXPOSE 8080

CMD ["/app/ciris-registry"]
