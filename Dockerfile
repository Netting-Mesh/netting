FROM rustlang/rust:nightly-alpine as builder

RUN apk add libressl-dev
RUN apk add musl-dev
RUN apk add protobuf

RUN USER=root cargo new --bin netting-builder 

WORKDIR /netting-builder/

COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY build.rs .

# build for release
RUN rm ./target/release/deps/netting*
RUN cargo build --release

# Strip debug symbols out of binary
RUN strip /netting-builder/target/release/netting

FROM debian:stable-slim

WORKDIR /netting

COPY --from=builder /netting-builder/target/release/netting /netting/netting

CMD /netting/netting

