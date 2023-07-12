use crate::PgPool;
use crate::jobs::check_for_new_releases::CheckForNewReleasesJob;
use crate::jobs::oshi_job::OshiJob;
use serenity::{model::channel::Message, prelude::*};

pub async fn exec(ctx: &Context, msg: &Message) {
    let content: String =
        format!("I'll check for new episodes right now. Will update you if I find anything! 😘");
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("version: error sending message: {:?}", why);
    }

    let data = ctx.data.read().await;
    let pool = data.get::<PgPool>().unwrap();

    CheckForNewReleasesJob::exec(&ctx.http, &pool).await;
}
