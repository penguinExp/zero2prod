FROM rust:1.67

WORKDIR /app

RUN apt-get update && apt-get install lld clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]
