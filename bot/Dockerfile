# Stage 1: Build the Rust application
FROM rust:1.88-slim-bullseye AS builder

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Set working directory to the bot directory
WORKDIR /usr/src/bot

# Copy the entire repository
COPY . .

# Ensure we're building from the bot directory
WORKDIR /usr/src/bot

# Build the release version
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/bot/target/release/neko-bot /usr/local/bin/neko-bot

# Run the application
CMD ["/usr/local/bin/neko-bot"]
