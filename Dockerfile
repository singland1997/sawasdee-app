FROM rust:1.95-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sawasdee-api /usr/local/bin/sawasdee-api
COPY --from=builder /app/migrations ./migrations
RUN mkdir -p ./uploads
EXPOSE 3000
CMD ["sawasdee-api"]