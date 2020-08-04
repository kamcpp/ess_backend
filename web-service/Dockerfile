FROM ubuntu:18.04 as builder

RUN apt update
RUN apt install -y build-essential libssl-dev openssl pkgconf curl libpq-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /work/src
WORKDIR /work

COPY Cargo.toml .
COPY Cargo.lock .
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

# -----------------------------------------
FROM ubuntu:18.04

RUN apt update
RUN apt install -y libssl-dev ca-certificates libpq-dev

COPY --from=builder /work/target/release/manning-simurgh-web-service /usr/bin/

ENTRYPOINT [ "manning-simurgh-web-service" ]
