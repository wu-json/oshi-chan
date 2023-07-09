## Oshi-chan

Oshi-chan is a Discord Bot that allows you to subscribe to anime releases (via 9anime.to) and get notified when they come out. I built this since my friends and I coordinate watch anime parties in Discord, and I thought it would be useful to have live notifications of new episodes coming out without having to constantly check the website manually.

Oshi is built entirely with Rust and lives on a single app (excluding the Postgres instance) deployed to [Fly.io](https://fly.io/dashboard). It scrapes 9anime for the shows in the watchlist every hour, and sends a message to the Discord channel when a new release is found.

## Development

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [libpq](https://formulae.brew.sh/formula/libpq): `brew install libpq`
- [Flyctl](https://fly.io/docs/flyctl/)

### Environment

When running Oshi-chan locally, you can use a `.env.development` file in the Cargo workspace root to add environment variables.

```env
# .env.development
DATABASE_URL=some-pg-url
DISCORD_BOT_TOKEN=some-discord-bot-token
OSHI_ENV=development
OSHI_DEVELOPMENT_CHANNEL_ID=some-discord-channel-id
OSHI_GENERAL_CHANNEL_ID=some-discord-channel-id
```

Note that `OSHI_DEVELOPMENT_CHANNEL_ID` can be used to specify a dedicated development discord channel that a production deployment of Oshi-Chan will ignore.

### Proxy Fly Postgres

While you can run a local Postgres instance for testing Oshi-chan locally, since this a "for-fun" project I usually just proxy the production Fly.io Postgres to my local machine.

```bash
fly proxy 5432 -a oshi-chan-pg
```

### Start the Program

You can start Oshi by running the following command. This should connect Oshi to Postgres, run any database migrations, and then connect the Discord listener and cron jobs for scraping watchlist entries.

```bash
ENV_FILE=.env.development cargo run -p oshi-chan
```

### Build the Docker Image

Oshi-chan is dockerized. If you want to build the image locally, you can run the following command.

```bash
docker build --platform linux/amd64 . -f oshi-chan/Dockerfile
```

## Usage

You can get a list of available commands by typing `!oshi` in the Discord channel. Oshi should respond to any channel except for `#oshi-development`, which is reserved for local testing.
