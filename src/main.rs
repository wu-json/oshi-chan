use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let oshi_env: String = env::var("OSHI_ENV").expect("OSHI_ENV is missing");
    match oshi_env.as_str() {
        "development" => {
            println!("Started Oshi-Chan in development mode");
            dotenv().ok();
            println!("Loaded environment variables from .env");
        }
        "production" => {
            println!("Started Oshi-Chan in production mode");
        }
        _ => panic!("OSHI_ENV={oshi_env} is not a valid environment"),
    }

    let framework = StandardFramework::new().configure(|c| c.prefix("~")); // set the bot's prefix to "~"

    let token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is missing");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    } 
}
