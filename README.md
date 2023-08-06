## Oshi-chan

Oshi-chan is a Discord Bot that allows you to subscribe to anime releases (via aniwave.to) and get notified when they come out. I built this since my friends and I coordinate anime watch parties in Discord, and I thought it would be useful to have live notifications of new episodes coming out without having to constantly check online manually.

Oshi is built entirely with Rust and lives on a single app (excluding the Postgres instance) deployed to [Fly.io](https://fly.io/dashboard). It scrapes aniwave for the shows in the watchlist every hour, and sends a message to the Discord channel when a new release is found.

![oshi-no-ko-twin](https://github.com/wu-json/oshi-chan/assets/45532884/fdf143e0-47bc-454e-b882-b661b0a7e180)

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
RELEASE_POLLING_PARALLELISM_LIMIT=some-usize
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

You can get a list of available commands by typing `!oshi` in the Discord channel.

![Screenshot 2023-07-09 at 4 07 10 PM](https://github.com/wu-json/oshi-chan/assets/45532884/e50473c4-e084-4521-a31a-a136c4c620fc)

Here's what adding a show to the watchlist looks like.

![Screenshot 2023-07-09 at 4 08 18 PM](https://github.com/wu-json/oshi-chan/assets/45532884/07c026b0-4c38-4fb8-9961-ec32da425420)

Here's what a notification from Oshi looks like.

![Screenshot 2023-07-09 at 4 09 05 PM](https://github.com/wu-json/oshi-chan/assets/45532884/053dae84-299a-4ce8-b286-70d2d1d2b689)
