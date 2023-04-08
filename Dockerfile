FROM rust:1.68 as build

COPY ./ ./

RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build --release --bin server

EXPOSE 50051
CMD ["./target/release/server"]
