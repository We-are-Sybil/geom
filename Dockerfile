FROM rust:bullseye as builder

RUN apt-get update && apt-get install -y lld clang && rm -rf /var/lib/apt/lists/*

RUN USER=root cargo new --bin geom
WORKDIR /geom

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.cargo ./.cargo

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/geom*
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /geom/target/release/geom .

ENV RUST_LOG=info
ENV TERM=xterm-256color

ENTRYPOINT ["./geom"]

CMD [ "--help" ]
