FROM rust:1.66

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 4000

CMD ["cargo", "run", "--release"]