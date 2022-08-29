FROM rust:1.61 AS builder

WORKDIR /to-do
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/rust-to-do ./target/release/rust-to-do
CMD ["/target/release/docker"]