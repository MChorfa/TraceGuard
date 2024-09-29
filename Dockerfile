# Build stage
FROM rust:1.60 as builder
WORKDIR /usr/src/traceguard
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/traceguard/target/release/traceguard /usr/local/bin/traceguard
EXPOSE 8080
CMD ["traceguard"]