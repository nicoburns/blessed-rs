FROM rust:1.59 as builder
WORKDIR /usr/src/blessed-rs
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./data ./data
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/blessed-rs /usr/local/bin/blessed-rs

WORKDIR /usr/blessed-rs
COPY ./static ./static
COPY ./templates ./templates
COPY ./data ./data
CMD ["blessed-rs"]