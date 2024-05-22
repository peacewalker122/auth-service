FROM rust:1.73.0-bullseye AS build

ARG APP_NAME=auth-service

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN cargo build --release

COPY src ./src
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/${APP_NAME}

CMD ["${APP_NAME}", "-p", "4221"]
