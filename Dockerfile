FROM rust:1.59 as builder



# Run dummy build to build and cache dependencies that only depends on Cargo.toml and Cargo.lock
WORKDIR /usr/src
RUN USER=root cargo new blessed-rs
COPY Cargo.toml Cargo.lock /usr/src/blessed-rs/
WORKDIR /usr/src/blessed-rs
RUN cargo build --release

# Run actual build
COPY ./src ./src
COPY ./data ./data
RUN cargo build --release

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/blessed-rs /usr/local/bin/blessed-rs

WORKDIR /usr/blessed-rs
COPY ./static ./static
COPY ./templates ./templates
COPY ./data ./data
CMD ["blessed-rs"]


WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new umar

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/umar/

WORKDIR /usr/src/umar

# This is a dummy build to get the dependencies cached.


# Now copy in the rest of the sources
COPY src /usr/src/umar/src/

# This is the actual build.
RUN cargo build --release \
    && mv target/release/umar /bin \
    && rm -rf /usr/src/umar

WORKDIR /
