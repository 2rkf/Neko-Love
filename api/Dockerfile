# Use the official Rust 1.88 slim image as the base for building
FROM rust:1.88-slim-bullseye AS builder

# Set the working directory
WORKDIR /usr/src/api

# Copy the project files
COPY ./api .

# Build the release version
RUN cargo build --release

# Create a new stage for the runtime
FROM debian:bullseye-slim

# Install any necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/api/target/release/neko-love-api /usr/local/bin/neko-love-api

# Expose the port Axum typically uses (adjust if your app uses a different port)
EXPOSE 3000

# Set the command to run the application
CMD ["neko-love-api"]
