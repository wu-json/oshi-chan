use std::env;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not defined");
    println!("{}", discord_token);
}

