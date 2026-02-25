# ---- Build Stage ----
FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/testing-rust /app/app

EXPOSE 8000

CMD ["./app"]