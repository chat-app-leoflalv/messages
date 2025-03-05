
FROM rust:1.85.0

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

RUN cargo install cargo-watch

RUN cargo build

EXPOSE 3000
