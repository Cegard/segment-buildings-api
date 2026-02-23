# ================================
# Builder Stage
# ================================
FROM docker.io/nvidia/cuda:12.6.2-devel-ubuntu24.04 AS builder

WORKDIR /app

# Install build dependencies (GCC 13, headers, etc.)
RUN apt-get update && \
  apt-get install -y \
  build-essential \
  cmake \
  pkg-config \
  libssl-dev \
  g++ \
  curl \
  git \
  ca-certificates \
  libclang-dev \
  ninja-build && \
  rm -rf /var/lib/apt/lists/*

# Install Rust (latest stable)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy project files
COPY . .

# Run tests to ensure code quality
RUN cargo test --release --features cuda

# Build the release binary with CUDA support
RUN cargo build --release --features cuda

# ================================
# Runtime Stage
# ================================
FROM docker.io/nvidia/cuda:12.6.2-runtime-ubuntu24.04

WORKDIR /app

# Install minimal runtime dependencies
RUN apt-get update && \
  apt-get install -y \
  ca-certificates \
  libssl3 \
  libstdc++6 \
  libc6 && \
  rm -rf /var/lib/apt/lists/*

# Copy Rust binary from builder
COPY --from=builder /app/target/release/buildings_digitizer /usr/local/bin/app

# Copy model files
COPY model ./model

ENV RUST_LOG=info
ENV LD_LIBRARY_PATH=/usr/local/cuda/lib64:${LD_LIBRARY_PATH}

# Expose port
EXPOSE 3000

# Run the binary
CMD ["app"]
