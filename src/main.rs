use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    framework::standard::StandardFramework,
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "hello oshi" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hello").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

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

    let framework: StandardFramework = StandardFramework::new();
    let token: String = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is missing");
    let intents: GatewayIntents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
