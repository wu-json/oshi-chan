FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release -p oshi-chan

FROM debian:bullseye-slim
RUN apt-get update && apt-get install libpq-dev -y
RUN useradd -ms /bin/bash app
USER app
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/oshi-chan /app/oshi-chan
CMD ./oshi-chan