FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/footy-alerts ./footy-alerts

ADD https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-amd64.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && apt-get install ca-certificates -y && update-ca-certificates

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/footy-alerts /app/footy-alerts
COPY --from=builder /usr/local/bin/litestream /usr/local/bin/litestream

# Copy Litestream configuration file & startup script.
COPY ./litestream.yml /etc/litestream.yml

# Run the app
CMD litestream replicate -exec "./footy-alerts"