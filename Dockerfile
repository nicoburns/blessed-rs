FROM debian:bullseye-slim
EXPOSE 3333

COPY target/release/blessed-rs /usr/local/bin/blessed-rs
RUN chmod +x /usr/local/bin/blessed-rs

WORKDIR /usr/blessed-rs
COPY ./static ./static
COPY ./data ./data


CMD ["blessed-rs"]