FROM rust:1.66

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]