mod commands;
mod environment;
mod handler;

use environment::{Environment, EnvironmentTrait};
use serenity::{framework::standard::StandardFramework, prelude::*};

#[tokio::main]
async fn main() {
    let oshi_env: String = Environment::init();
    println!("Starting oshi-chan in {oshi_env} environment");

    let framework: StandardFramework = StandardFramework::new();
    let token: String = Environment::get_discord_token();

    let intents: GatewayIntents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating serenity client");

    if let Err(why) = client.start().await {
        println!("Serenity client error: {:?}", why);
    }
}
