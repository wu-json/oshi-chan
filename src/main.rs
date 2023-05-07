mod environment;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    framework::standard::StandardFramework,
    prelude::*,
};

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
    environment::init();

    let framework: StandardFramework = StandardFramework::new();
    let token: String = environment::get_discord_token();

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
