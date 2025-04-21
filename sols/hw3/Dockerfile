FROM rust:latest

WORKDIR /usr/src/hw3
COPY . .

RUN apt-get update && \
	apt-get install -y --no-install-recommends \
	libfontconfig1-dev \
	pkg-config \
	build-essential \
	&& rm -rf /var/lib/apt/lists/*

RUN cargo build --release

CMD ["/usr/src/hw3/target/release/hw3", "q5", "q8"]

