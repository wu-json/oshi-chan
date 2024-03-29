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
            "poll" => {
                commands::poll::exec(&ctx, &msg).await;
            }
            "version" => {
                commands::version::exec(&ctx, &msg).await;
            }
            "watchlist" => {
                if command_parts.len() < 3 {
                    return;
                }
                match command_parts[2] {
                    "add" => {
                        if command_parts.len() < 5 {
                            return;
                        }
                        commands::watchlist_add::exec(
                            &ctx,
                            &msg,
                            command_parts[3],
                            command_parts[4],
                        )
                        .await;
                    }
                    "delete" => {
                        if command_parts.len() < 4 {
                            return;
                        }
                        commands::watchlist_delete::exec(&ctx, &msg, command_parts[3]).await;
                    }
                    "destroy" => {
                        commands::watchlist_destroy::exec(&ctx, &msg).await;
                    }
                    "list" => {
                        commands::watchlist_list::exec(&ctx, &msg).await;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Handler for {} is connected", ready.user.name);
    }
}
