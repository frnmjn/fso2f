FROM rust:1.89

RUN apt-get update && apt-get install -y build-essential clang cmake perl pkg-config && rm -rf /var/lib/apt/lists/*

WORKDIR /app
