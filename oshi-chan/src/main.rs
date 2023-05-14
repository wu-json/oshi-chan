mod commands;
mod environment;
mod handler;

use environment::{Environment, EnvironmentTrait};
use pg_client::{ConnectionManager, PgConnection, Pool};
use serenity::{framework::standard::StandardFramework, prelude::*};

pub struct PgPool;

impl serenity::prelude::TypeMapKey for PgPool {
    type Value = Pool<ConnectionManager<PgConnection>>;
}

#[tokio::main]
async fn main() {
    let oshi_env: String = Environment::init();
    println!("Starting oshi-chan in {oshi_env} environment");

    let framework: StandardFramework = StandardFramework::new();
    let token: String = Environment::get_discord_token();

    let intents: GatewayIntents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let pool: Pool<ConnectionManager<PgConnection>> =
        pg_client::create_connection_pool(&Environment::get_database_url());
    {
        let connection: &mut pg_client::PooledConnection<ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        pg_client::migrate(connection);
    };

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating serenity client");

    {
        let mut data = client.data.write().await;
        data.insert::<PgPool>(pool);
    }

    if let Err(why) = client.start().await {
        println!("Serenity client error: {:?}", why);
    }
}
