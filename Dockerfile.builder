FROM ubuntu:18.04 as builder

RUN apt update
RUN apt install -y build-essential libssl-dev openssl pkgconf curl libpq-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /work

ENTRYPOINT [ "cargo" ]
