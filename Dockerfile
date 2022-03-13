FROM rust:1.59 as builder
WORKDIR /usr/src/blessed-rs
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/blessed-rs /usr/local/bin/blessed-rs
CMD ["blessed-rs"]