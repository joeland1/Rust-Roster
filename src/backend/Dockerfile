FROM rust:latest

workdir /root

COPY . .

RUN cargo build

EXPOSE 8000

CMD ["cargo","run"]