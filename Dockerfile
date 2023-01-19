FROM rust:1.66-alpine as builder

RUN USER=root cargo new --bin doctor-service
WORKDIR /doctor-service

COPY . . 

RUN cargo build --release

CMD ["./taget/release/doctor-service"]