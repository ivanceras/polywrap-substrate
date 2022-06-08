# using 2 build step
FROM rust:1.60.0-slim-bullseye as build

RUN apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config clang
RUN cargo install wasm-pack
COPY . .
RUN cargo build --release -p server
RUN wasm-pack build --release --target web examples/mycelium_usage
RUN cargo build --release --target wasm32-unknown-unknown -p mycelium


# The actual server image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates
EXPOSE 3030
COPY --from=build ./target/release/server /usr/bin/server
CMD /usr/bin/server

