FROM rust:1.88 AS build

WORKDIR /app

RUN apt-get update && apt-get install -y \
    musl-tools pkg-config libssl-dev \
    && rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch AS runtime
ARG LISTEN_PORT=8080
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/mangayomi-server /app/server
COPY ./resources ./resources
EXPOSE ${LISTEN_PORT}
CMD ["/app/server"]
