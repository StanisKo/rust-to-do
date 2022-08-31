FROM rust:1.61

WORKDIR /to-do
COPY . .

RUN cargo install cargo-watch