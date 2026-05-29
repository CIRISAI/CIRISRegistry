# CIRISRegistry — multi-stage build (root Dockerfile, used by CI per
# `.github/workflows/docker.yml` with `context: .`).
#
# v2.0.0+ workspace layout:
#   rust-registry/
#   ├── Cargo.toml (workspace)
#   ├── Cargo.lock
#   ├── ciris-registry-core/ (lib; build.rs + migrations + src)
#   └── ciris-registry/      (bin; src/main.rs)
#
# An identical Dockerfile exists at rust-registry/Dockerfile for
# operators who want to docker-build from inside the rust-registry/
# directory; keep both in sync.

# Builder stage
FROM rust:1.93-bookworm AS builder

# System dependencies:
# - protobuf-compiler / libprotobuf-dev: tonic-build proto codegen
# - libtss2-dev + pkg-config: ciris-keyring's `tpm` feature (pulled
#   transitively by ciris-persist v3.3.1 which enables tpm on Linux);
#   without this, tss-esapi-sys's build.rs fails with "pkg-config could
#   not find tss2-sys >= 2.4.6"
# - libssl-dev: openssl-sys (transitively pulled by some deps)
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    libtss2-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Proto file at /protocol/ — build.rs in ciris-registry-core/ resolves it
# via "../../protocol/" (two levels up from the lib's Cargo.toml: out of
# ciris-registry-core/ then out of /app to reach /protocol/).
COPY protocol/ciris_registry.proto /protocol/

# Workspace root files first (for dep cache layer).
COPY rust-registry/Cargo.toml rust-registry/Cargo.lock* ./

# Member Cargo.tomls + the lib's build.rs (needed even for stub-build
# because the proto generation runs during `cargo build`).
COPY rust-registry/ciris-registry-core/Cargo.toml ./ciris-registry-core/Cargo.toml
COPY rust-registry/ciris-registry-core/build.rs ./ciris-registry-core/build.rs
COPY rust-registry/ciris-registry/Cargo.toml ./ciris-registry/Cargo.toml

# Stub sources so cargo can resolve + fetch + build deps without the
# actual source code yet. The subsequent COPY of the real sources will
# invalidate this layer's cache; deps are already compiled.
RUN mkdir -p ciris-registry-core/src ciris-registry/src && \
    echo "// stub for dep caching — replaced by COPY below" > ciris-registry-core/src/lib.rs && \
    echo "fn main() {}" > ciris-registry/src/main.rs

# Build deps only. May fail on the stub source (lib.rs is empty so
# `pub use` re-exports etc. won't resolve); we don't care — we want the
# dep .rlib cache, not a successful binary.
RUN cargo build --release --workspace 2>/dev/null || true

# Now copy the real source. Each member's source goes to the right place.
COPY rust-registry/ciris-registry-core/src ./ciris-registry-core/src
COPY rust-registry/ciris-registry-core/migrations ./ciris-registry-core/migrations
COPY rust-registry/ciris-registry/src ./ciris-registry/src

# Touch entry points so cargo re-runs the build with the real source.
RUN touch ciris-registry-core/src/lib.rs ciris-registry/src/main.rs && \
    cargo build --release --workspace

# Runtime stage
FROM debian:bookworm-slim

# Runtime libs:
# - ca-certificates: TLS to upstream Persist / Edge / Verify
# - libtss2-dev: pulls all the runtime TPM .so libs that ciris-keyring's
#   tpm-feature code dlopens. Using the -dev package is heavier than
#   strictly necessary (~3MB extra over individual runtime libs) but
#   reliable across Debian point releases — the individual runtime
#   package names (libtss2-esys-X.X.X-N etc.) drift between bookworm
#   minor versions, and a prior attempt with explicit version-suffixed
#   names broke when one of them (libtss2-tcti-device-0-0) didn't
#   exist in bookworm-slim's package set.
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libtss2-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/ciris-registry /app/ciris-registry

# Copy migrations (now under the lib crate's directory)
COPY --from=builder /app/ciris-registry-core/migrations /app/migrations

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
