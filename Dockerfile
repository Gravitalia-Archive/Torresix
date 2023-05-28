FROM rust:1.69 as build

RUN USER=root cargo new --bin torresix
WORKDIR /torresix

COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./proto ./proto
COPY ./models ./models
COPY ./src ./src

RUN apt-get update && apt-get install -y libssl-dev pkg-config protobuf-compiler

RUN cargo build --release --bin server

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl3
RUN apt-get install -y libc6

COPY --from=build /torresix/target/release/server .

EXPOSE 50051
CMD ["./server"]
