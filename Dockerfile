#  format2 — Multi-stage Docker build
# ---------------------------------------------------------------------------
# Stage 1: Build all binaries in release mode
# ---------------------------------------------------------------------------
FROM rust:1.93-bookworm AS builder
WORKDIR /build
# Copy the entire project
COPY . .
# Build the project in release mode
RUN cargo build --release

# ---------------------------------------------------------------------------
# Stage 2: Minimal runtime image
# ---------------------------------------------------------------------------
FROM ubuntu:24.04

# RUN apt-get update && apt-get install -y --no-install-recommends \
#     ca-certificates \
#     && rm -rf /var/lib/apt/lists/*

# Copy both binaries
COPY --from=builder /build/target/release/format2 /usr/local/bin/format2

# Default: run the CLI help command
CMD ["format2", "--help"]