FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/oshi-chan ./oshi-chan

FROM debian:bullseye-slim
RUN useradd -ms /bin/bash app
USER app
WORKDIR /app
COPY --from=builder /usr/src/app/oshi-chan /app/oshi-chan
CMD ./oshi-chan