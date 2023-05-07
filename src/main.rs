use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Hello, world!");
    // let discord_token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_TOKEN is not defined");
    // println!("{}", discord_token);
}

