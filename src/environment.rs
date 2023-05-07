use dotenv::from_filename;
use serenity::model::id::ChannelId;
use std::env;

pub fn init() -> String {
    let env_file: Result<String, env::VarError> = env::var("ENV_FILE");
    if let Ok(file) = env_file {
        from_filename(&file).ok();
        println!("Loaded environment variables from {file}");
    }
    get_oshi_env()
}

pub fn get_discord_token() -> String {
    env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is missing")
}

pub fn get_oshi_env() -> String {
    env::var("OSHI_ENV").expect("OSHI_ENV is missing")
}

pub fn get_oshi_testing_channel_id() -> ChannelId {
    let env_str: String = env::var("OSHI_TESTING_CHANNEL_ID").expect("OSHI_TESTING_CHANNEL_ID");
    ChannelId(env_str.parse::<u64>().unwrap())
}
