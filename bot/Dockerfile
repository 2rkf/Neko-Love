# Use Rust official image for building
FROM rust:1.82 AS builder

WORKDIR /app

# Copy dependency files to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY . .
RUN cargo build --release

# Final stage: minimal runtime image
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/neko-bot /usr/local/bin/neko-bot

WORKDIR /app

ENTRYPOINT ["/usr/local/bin/neko-bot"]