# Stage 1: Build the Rust binary
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

# Install dependencies (if needed)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/hello_world /app/rust-threadpool-server

# Copy static files (HTML, etc.)
COPY hello.html /app/hello.html
COPY 404.html /app/404.html

# Expose the port the server listens on
EXPOSE 7878

# Set the entry point to run the server
ENTRYPOINT ["/app/rust-threadpool-server"]