FROM ubuntu:18.04 as builder

RUN apt update
RUN apt install -y build-essential libssl-dev openssl pkgconf curl libpq-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /src
COPY . /src/
WORKDIR /src

RUN cargo build --release

# -----------------------------------------
FROM ubuntu:18.04

RUN apt update
RUN apt install -y libssl-dev ca-certificates libpq-dev

COPY --from=builder /src/target/release/manning-simurgh-web-service /usr/bin/

ENTRYPOINT [ "manning-simurgh-web-service" ]
