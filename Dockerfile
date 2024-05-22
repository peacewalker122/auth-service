FROM rust:1.73-slim as builder
RUN USER=root cargo new --bin http-server-starter-rust

COPY . /app
WORKDIR /app

RUN apk add --no-cache wget

RUN wget https://github.com/golang-migrate/migrate/releases/download/v4.17.1/migrate.linux-amd64.tar.gz
RUN tar -xzvf migrate.linux-amd64.tar.gz

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
WORKDIR /app
COPY --from=builder /app/migrate /usr/bin/migrate
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/http-server-starter-rust ./server

EXPOSE 4221

CMD [ "/app/server","-p","4221"]
