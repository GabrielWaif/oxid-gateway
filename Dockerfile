FROM rust:1.79 AS build

WORKDIR /build

COPY src/ src/
COPY migrations/ migrations/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY diesel.toml diesel.toml

RUN cargo build --locked --release
RUN apt-get update && apt install -y openssl libpq-dev libc6

RUN cp ./target/release/oxid-gateway /bin/server
CMD ["/bin/server"]
