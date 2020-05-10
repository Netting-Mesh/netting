FROM rustlang/rust:nightly as builder

RUN apt-get update\
    && yes yes | apt-get install cmake \
    && yes yes | apt-get install protobuf-compiler

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

RUN apt-get update\
	&& yes yes | apt-get install openssl

WORKDIR /netting

COPY --from=builder /netting-builder/target/release/netting /netting/netting

CMD /netting/netting

