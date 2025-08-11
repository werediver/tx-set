FROM mcr.microsoft.com/devcontainers/rust:latest AS build

RUN apt-get update && apt-get install -y musl-tools && rustup target add x86_64-unknown-linux-musl
