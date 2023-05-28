FROM rust:1.69-slim as builder

RUN USER=root cargo new --bin torresix
WORKDIR /torresix

COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./proto ./proto
COPY ./models ./models
COPY ./src ./src

RUN apt-get update && apt-get install -y libssl-dev pkg-config protobuf-compiler

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release --bin server

FROM debian:latest

COPY --from=build /torresix/target/release/server .

EXPOSE 50051
CMD ["./server"]
