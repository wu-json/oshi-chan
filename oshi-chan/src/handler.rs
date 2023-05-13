use crate::commands;
use crate::environment::{Environment, EnvironmentTrait};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore #oshi-development channel in production since it's for local testing
        if Environment::get_oshi_env() == "production"
            && Environment::get_oshi_dev_channel_id() == msg.channel_id
        {
            return;
        // Ignore any messages not meant for oshi-chan
        } else if !msg.content.starts_with("!oshi") {
            return;
        }

        let content_copy: String = msg.content.clone();
        let parts: std::str::Split<&str> = content_copy.split(" ");
        let command_parts: Vec<&str> = parts.collect::<Vec<&str>>();

        if command_parts.len() < 2 {
            commands::introduce::exec(&ctx, &msg).await;
            return;
        }

        match command_parts[1] {
            "version" => {
                commands::version::exec(&ctx, &msg).await;
            }
            _ => (),
        }

        if msg.content == "hello oshi" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hello").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Handler for {} is connected", ready.user.name);
    }
}
