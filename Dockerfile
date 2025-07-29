FROM rust:1.85 AS builder

WORKDIR /app

# FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y clang && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY denali/src ./denali/src
COPY denali/Cargo.toml ./denali
COPY example_contracts/vote/src ./example_contracts/vote/src
COPY example_contracts/vote/Cargo.toml ./example_contracts/vote
COPY macros/src ./macros/src
COPY macros/Cargo.toml ./macros
COPY load-test/src ./load-test/src
COPY load-test/Cargo.toml ./load-test
COPY Cargo.toml ./

RUN cd denali && \
        cargo build --release --package denali --package vote

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/denali .
RUN mkdir ./plugins
COPY --from=builder /app/target/release/libvote.so ./plugins

CMD ["./denali"]
