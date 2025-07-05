FROM rust:bookworm AS build

WORKDIR /app

# Replace "x86_64-unknown-linux-musl" with "aarch64-unknown-linux-musl" if running on arm64 CPU

RUN apt-get update && apt-get install -y \
    musl-tools pkg-config libssl-dev \
    && rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

EXPOSE 8080/tcp

FROM scratch

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/mangayomi-server /app/server
CMD ["/app/server"]
