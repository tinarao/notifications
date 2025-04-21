# syntax=docker/dockerfile:1

# --- Builder Stage ---
FROM rust:latest AS builder
WORKDIR /usr/src/notificator

COPY Cargo.toml Cargo.lock ./
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir src

COPY . .
RUN cargo build --release

# --- Final Stage ---
FROM debian:bookworm-slim

RUN apt-get install 

RUN apt update
RUN apt install libc6

RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
# Copy binary from builder
COPY --from=builder /usr/src/notificator/target/release/notificator .

ENV PORT=3692
EXPOSE 3692

# Healthcheck endpoint
HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:${PORT}/hc || exit 1

# Run the application
CMD ["./notificator"]
