## Fly Postgres

This is the Fly configuration for the Postgres instance used with Oshi-Chan. To connect to the database for local development, you will need to proxy the instance to your machine:

```bash
fly proxy 5432 -a oshi-chan-pg
```

From there you can connect to it via localhost using Oshi-Chan or TablePlus.