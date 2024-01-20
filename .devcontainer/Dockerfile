FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
RUN apt-get -qq update
RUN apt-get -qq install -y gcc-mingw-w64-x86-64 gcc-mingw-w64-i686
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add x86_64-pc-windows-gnu
