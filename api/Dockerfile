# Stage 1: Build the Rust application
FROM rustlang/rust:nightly AS builder

# Create a new empty shell project
WORKDIR /usr/src/app
RUN USER=root cargo new --bin neko-love-api
WORKDIR /usr/src/app/neko-love-api

# Copy over the manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN cargo build --release

# Final stage using a slim image
FROM debian:buster-slim

# Install OpenSSL - it's dynamically linked by some of our dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/neko-love-api/target/release/neko-love-api /usr/local/bin/neko-love-api

# Set the environment variable for production
# ENV RUST_LOG=info
# ENV APP_ENVIRONMENT=production

# Expose the port the app runs on
EXPOSE 3030

# Run the binary
CMD ["neko-love-api"]