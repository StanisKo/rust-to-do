FROM rust:1.61 AS builder

WORKDIR /to-do
COPY . .

RUN cargo build --release
RUN strip target/release/rust-to-do

FROM debian:buster-slim

COPY --from=builder /to-do/target/release/rust-to-do .

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

ENTRYPOINT ["/rust-to-do"]