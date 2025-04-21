FROM rust:latest

WORKDIR /usr/src/hw4
COPY . .

RUN cargo build --release

CMD ["/usr/src/hw4/target/release/hw4", "q2", "q6"]

