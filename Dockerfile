FROM rust:1.61

WORKDIR /to-do
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.7.3/wait /wait
RUN chmod +x /wait