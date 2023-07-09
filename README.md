## Oshi-Chan
Oshi-chan is a bot for the Tengoku Discord Server. This is a work in progress and I am still ass at Rust so we will see how this goes.

## Development

### Set Up Environment
You will need to get a `.env.development` file from Jason to get started, and then place it in the project root.

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install libpq
This is the C application interface with PostgreSQL, and is necessary for running Oshi locally.

```bash
# note, you may need to update ~/.zprofile or ~/.zshrc after doing this
brew install libpq
```

### Build and Start the Program
```bash
# proxy flyio postgres to local
fly proxy 5432 -a oshi-chan-pg

# start oshi
ENV_FILE=.env.development cargo run -p oshi-chan
```

### Build the Docker Image
Oshi-chan is dockerized and deployed on Fly.io. If you want to build the container locally, you can run. Note that the `--platform` argument is very important if you are trying to do this on an M1 Mac, otherwise you may be building with the wrong architecture.

```bash
docker build --platform linux/amd64 . -f oshi-chan/Dockerfile
```

## Usage

You can get a list of available commands by typing `!oshi` in the Discord channel. Oshi should respond to any channel except for `#oshi-development`, which is reserved for local testing.

## Feature Backlog

- Birthday reminders
- Music player