FROM rust:latest

WORKDIR /usr/src/hw2
COPY . .

RUN cargo build --release

CMD ["/usr/src/hw2/target/release/hw2", "q4", "q5"]

