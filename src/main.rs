use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let oshi_env: String = env::var("OSHI_ENV").expect("OSHI_ENV is missing");
    if oshi_env == "development" {
        println!("Started Oshi-Chan in development mode");
        dotenv().ok();
        println!("Loaded environment variables from .env");
    }

    println!("Hello, world!");

    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is missing");

    println!("{}", discord_token);
}
