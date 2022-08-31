FROM rust:1.61 AS builder

WORKDIR /to-do
COPY . .

RUN cargo install cargo-watch