## Oshi-Chan
Oshi-chan is a bot for the Tengoku Discord Server. This is a work in progress and I am still ass at Rust so we will see how this goes.

## Development

### Set Up Environment
You will need to get a `.env.development` file from Jason to get started, and then place it in the project root.

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build and Start the Program
```bash
ENV_FILE=.env.development cargo run
```