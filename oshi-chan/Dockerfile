FROM rust:1.75.0-bullseye as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release -p oshi-chan

FROM debian:bullseye-slim

# for packages
RUN apt-get update

# for diesel pg client
RUN apt-get install -y libpq-dev

# for headless chrome
RUN apt-get install -y wget
RUN wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt-get install -y ./google-chrome-stable_current_amd64.deb

RUN useradd -ms /bin/bash app
USER app
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/oshi-chan /app/oshi-chan
CMD ./oshi-chan
