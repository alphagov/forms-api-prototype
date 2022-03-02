FROM rust

# We create a new empty shell project to fake the dependencies so we can cache them
WORKDIR /code
RUN USER=root cargo new --bin rust-api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo install --path .

CMD ["rust-api"]
