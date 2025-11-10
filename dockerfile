# Build Stage
FROM rust:1.91-slim-bullseye AS builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=${DATABASE_URL}

COPY . .

RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api_test .

CMD ["./api_test"]

