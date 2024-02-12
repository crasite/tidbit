FROM rust:1.76-slim

WORKDIR /usr/src/myapp
COPY target/aarch64-unknown-linux-gnu/debug/otel /usr/src/myapp/otel


ENTRYPOINT ["./otel"]
