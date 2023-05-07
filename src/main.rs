use dotenv::dotenv;
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

    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is missing");
    println!("{}", discord_token);
}
