FROM debian:buster-slim
COPY target/release/blessed-rs /usr/local/bin/blessed-rs

WORKDIR /usr/blessed-rs
COPY ./static ./static
COPY ./templates ./templates
COPY ./data ./data

RUN chmod +x /usr/local/bin/blessed-rs

CMD ["blessed-rs"]