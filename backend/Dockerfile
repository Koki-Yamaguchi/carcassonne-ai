FROM registry.gitlab.com/rust_musl_docker/image:stable-latest as builder
WORKDIR /usr/src/app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/carcassonne /carcassonne
COPY --from=builder /usr/src/app/Rocket.toml /
CMD ["/carcassonne"]
EXPOSE 8000

