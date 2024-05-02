FROM rust:1.69.0 AS build-stage

RUN cargo new --bin rust-todo-be
WORKDIR /rust-todo-be

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./build.rs ./build.rs
RUN touch src/main.rs
RUN cargo build --release


FROM debian:bullseye-slim
COPY --from=build-stage /rust-todo-be/target/release/rust-todo-be /

RUN apt update
RUN apt install -y libssl-dev

CMD ["/rust-todo-be"]
