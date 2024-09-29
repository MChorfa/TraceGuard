FROM rust:1.60 as builder
WORKDIR /usr/src/traceguard
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/traceguard/target/release/traceguard /usr/local/bin/traceguard
CMD ["traceguard"]