use serenity::{
    model::channel::Message,
    prelude::*,
};

pub async fn exec(ctx: &Context, msg: &Message)  {
    let pkg_version: &str = env!("CARGO_PKG_VERSION");
    let content: String = format!("I'm on v{pkg_version}. Thanks for asking owo!");
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("version: error sending message: {:?}", why);
    }
}