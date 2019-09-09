FROM ubuntu:bionic

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH


RUN apt update
RUN apt install -y curl vim build-essential libarchive pkg-config libssl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path

RUN cargo install cargo-deb
