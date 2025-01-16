FROM rust:latest

WORKDIR /usr/src/hw1
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--", "q3"]

