use dotenv::from_filename;
use serenity::model::id::ChannelId;
use std::env;

#[cfg_attr(test, mockall::automock)]
pub trait EnvironmentTrait {
    fn init() -> String;
    fn get_database_url() -> String;
    fn get_discord_token() -> String;
    fn get_oshi_env() -> String;
    fn get_oshi_dev_channel_id() -> ChannelId;
    fn get_oshi_general_channel_id() -> ChannelId;
    fn get_oshi_version() -> &'static str;
    fn get_release_polling_parallelism_limit() -> usize;
}

pub struct Environment;

impl EnvironmentTrait for Environment {
    fn init() -> String {
        let env_file: Result<String, env::VarError> = env::var("ENV_FILE");
        if let Ok(file) = env_file {
            from_filename(&file).ok();
            println!("Loaded environment variables from {file}");
        }
        Environment::get_oshi_env()
    }

    fn get_database_url() -> String {
        env::var("DATABASE_URL").expect("DATABASE_URL is missing")
    }

    fn get_discord_token() -> String {
        env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is missing")
    }

    fn get_oshi_env() -> String {
        env::var("OSHI_ENV").expect("OSHI_ENV is missing")
    }

    fn get_oshi_general_channel_id() -> ChannelId {
        let env_str: String =
            env::var("OSHI_GENERAL_CHANNEL_ID").expect("OSHI_DEVELOPMENT_CHANNEL_ID");
        ChannelId(env_str.parse::<u64>().unwrap())
    }

    fn get_oshi_dev_channel_id() -> ChannelId {
        let env_str: String =
            env::var("OSHI_DEVELOPMENT_CHANNEL_ID").expect("OSHI_DEVELOPMENT_CHANNEL_ID");
        ChannelId(env_str.parse::<u64>().unwrap())
    }

    fn get_oshi_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn get_release_polling_parallelism_limit() -> usize {
        match env::var("RELEASE_POLLING_PARALLELISM_LIMIT") {
            Ok(v) => v.parse::<usize>().unwrap(),
            Err(_) => 2,
        }
    }
}
