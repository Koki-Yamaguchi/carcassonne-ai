FROM registry.gitlab.com/rust_musl_docker/image:stable-latest as builder
WORKDIR /usr/src/app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/carcassonne /carcassonne
COPY --from=builder /usr/src/app/Rocket.toml /

COPY --from=builder /usr/src/app/self-signed.cert /usr/local/share/ca-certificates/extra/self-signed.crt
RUN apt-get update -y && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/* && update-ca-certificates

CMD ["/carcassonne"]
EXPOSE 8000

