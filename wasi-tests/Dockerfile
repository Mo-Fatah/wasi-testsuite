FROM ubuntu:16.04

ARG USER=root
# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"


WORKDIR /usr/src/

COPY . .

RUN rustc --version
RUN rustup target add wasm32-wasi

RUN cargo build --target wasm32-wasi
