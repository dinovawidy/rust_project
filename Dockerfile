# -----------------------------
# Build Stage
# -----------------------------
FROM rust:1.92-slim as builder

WORKDIR /app

COPY . .

# 🔥 SQLX offline mode
ENV SQLX_OFFLINE=true

RUN cargo build --release


# -----------------------------
# Runtime Stage
# -----------------------------
FROM debian:bookworm-slim

WORKDIR /app

# install minimal runtime deps
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/testing-rust /app/app

EXPOSE 8000

CMD ["./app"]