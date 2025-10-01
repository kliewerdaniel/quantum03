FROM rust:1.86-slim as builder

WORKDIR /usr/src/app

COPY Cargo.toml .
COPY src ./src

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/quantum-messenger-server .

EXPOSE 8080

CMD ["./quantum-messenger-server"]
