# Base image for Rust
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/traceguard

# Copy the project files
COPY . .

# Build the project in release mode
RUN cargo build --release

# Final base image for runtime
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built binary from the builder
COPY --from=builder /usr/src/traceguard/target/release/traceguard .

# Install any additional dependencies (if needed)
RUN apt-get update && apt-get install -y libpq-dev && apt-get clean

# Expose the port the app will run on
EXPOSE 8080

# Run the app
CMD ["./traceguard"]
