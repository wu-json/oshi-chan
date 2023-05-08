use crate::commands;
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
        if super::environment::get_oshi_env() == "production"
            && super::environment::get_oshi_dev_channel_id() == msg.channel_id
        {
            return;
        // Ignore any messages not meant for oshi-chan
        } else if !msg.content.starts_with("!oshi") {
            return;
        }

        let content_copy = msg.content.clone();
        let parts = content_copy.split(" ");
        let command_parts = parts.collect::<Vec<&str>>();

        if command_parts.len() < 2 {
            // TODO: put help script here
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
