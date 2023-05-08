FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build -p oshi-chan --release

FROM debian:bullseye-slim
RUN useradd -ms /bin/bash app
USER app
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/oshi-chan /app/oshi-chan
CMD ./oshi-chan