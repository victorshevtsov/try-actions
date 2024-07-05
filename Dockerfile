FROM rust:1.78 as builder

WORKDIR /usr/src/verity
COPY . .

RUN cargo install


FROM debian:bookworm-slim

RUN \
  apt update && \
  apt install -y openssl && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/try-actions /usr/local/bin/try-actions

CMD ["try-actions"]
