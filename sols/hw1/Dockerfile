FROM rust:latest

WORKDIR /usr/src/hw1
COPY . .

RUN cargo build --release

CMD ["/usr/src/hw1/target/release/hw1", "q3"]

