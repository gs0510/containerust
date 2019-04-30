FROM rust:1.31


COPY . /Users/gargisharma/container
WORKDIR /Users/gargisharma/container
COPY . .


RUN cargo install --path .

RUN cargo run run / /