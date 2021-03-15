FROM rust:latest

WORKDIR /usr/src/umi
COPY . .

RUN cargo install --path .

CMD ["umi"]
