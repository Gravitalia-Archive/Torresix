FROM rust:1.69 as build

RUN apt-get update && apt-get install -y wget
RUN wget https://ftp.gnu.org/gnu/libc/glibc-2.37.tar.gz

RUN tar -xvf glibc-2.37.tar.gz
WORKDIR glibc-2.37

RUN mkdir build
WORKDIR build

RUN ../configure --prefix=/usr
RUN make
RUN make install

WORKDIR /

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

COPY --from=build /torresix/target/release/server .

EXPOSE 50051
CMD ["./server"]
