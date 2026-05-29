FROM rust:1.84

RUN apt-get update && apt-get install -y build-essential clang cmake perl pkg-config && rm -rf /var/lib/apt/lists/*
RUN rustup default nightly

WORKDIR /app
