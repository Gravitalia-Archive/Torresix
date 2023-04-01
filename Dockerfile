FROM rust:1.63 as build

COPY ./ ./

RUN cargo build --release --bin server

CMD ["./target/release/server"]