FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends npm \
                                               curl \
                                               build-essential \
                                               pkg-config \
                                               git \
                                               clang \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y -t wasm32-unknown-unknown --profile minimal
RUN bash -c "source $HOME/.cargo/env && cargo install wasm-pack"

WORKDIR /src