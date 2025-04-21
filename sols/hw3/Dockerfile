FROM rust:latest

WORKDIR /usr/src/hw3
COPY . .

RUN cargo build --release

CMD ["/usr/src/hw3/target/release/hw3", "q5", "q8"]

