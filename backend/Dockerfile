FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/footy-alerts ./footy-alerts


# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && apt-get install ca-certificates -y && update-ca-certificates

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/footy-alerts /app/footy-alerts

# Run the app
CMD ./footy-alerts