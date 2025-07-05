FROM rust:bookworm AS build

WORKDIR /app

RUN apt-get update && apt-get install -y \
    musl-tools pkg-config libssl-dev \
    && rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/mangayomi-server /app/server
CMD ["/app/server"]
