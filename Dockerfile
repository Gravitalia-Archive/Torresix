FROM rust:1.69 as build

RUN USER=root cargo new --bin torresix
WORKDIR /torresix

COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY --from=build proto/ proto/
COPY --from=build /models /models

RUN cargo build --release \
 && rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/torresix* \
 && cargo build --release --bin server

FROM rust:1.69-slim-buster

COPY --from=build /torresix/target/release/server .

EXPOSE 50051
CMD ["./server"]
