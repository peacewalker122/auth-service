FROM rust:1.73-bullseye AS build

ARG APP_NAME=auth-service

RUN apt-get update && apt-get install -y cargo

COPY . /app
WORKDIR /app

RUN cargo build --release

COPY src ./src
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/${APP_NAME}

CMD ["${APP_NAME}", "-p", "4221"]
