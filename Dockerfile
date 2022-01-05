FROM rust

RUN apt update && apt install -y strace
