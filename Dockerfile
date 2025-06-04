# ---- Stage 1: Python + Build Tools ----
FROM alpine:3.18 AS python-builder

# Install Python 3.10 and build tools
RUN apk add --no-cache \
    python3 \
    py3-pip \
    py3-setuptools \
    py3-wheel \
    py3-virtualenv \
    bash \
    build-base \
    boost-dev \
    libarchive-dev \
    rapidjson-dev \
    linux-headers \
    bash \
    curl

# Ensure python points to Python 3
RUN ln -sf python3 /usr/bin/python && ln -sf pip3 /usr/bin/pip

# ---- Stage 2: Build Rust Project ----
FROM rust:1.87-alpine AS builder

# Install required dependencies for building Rust crates that depend on C libs
RUN apk add --no-cache \
    build-base \
    protobuf-dev \
    bash \
    curl \
    openssl-dev \
    pkgconfig \
    libarchive-dev \
    boost-dev

WORKDIR /usr/src/galemind
COPY . .
RUN cargo build && cargo install --path src/galemind

# ---- Stage 3: Final runtime image ----
FROM alpine:3.18

# Install runtime dependencies only
RUN apk add --no-cache \
    python3 \
    py3-pip \
    libstdc++ \
    protobuf \
    boost \
    libarchive \
    rapidjson \
    bash \
    curl

# Create symlinks to standardize Python binary naming
RUN ln -sf python3 /usr/bin/python && ln -sf pip3 /usr/bin/pip

# Copy the compiled Rust binary
COPY --from=builder /usr/local/cargo/bin/galemind /usr/local/bin/galemind
EXPOSE 8080
EXPOSE 50051
# Default command
CMD ["galemind", "start"]
