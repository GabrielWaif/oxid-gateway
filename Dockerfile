FROM rust:1.79 AS build

ARG APP_NAME=oxid-gateway
ENV DATABASE_URL=postgres://postgres:admin@oxid-gateway-postgres:5432/postgres

WORKDIR /build

COPY . .

RUN rm .env

RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

RUN apt-get update && apt install -y openssl libpq-dev libc6

CMD ["/bin/server"]
