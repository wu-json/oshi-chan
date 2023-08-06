use crate::PgPool;
use pg_client::{models, ConnectionManager, PgConnection, Pool, PooledConnection};
use scrape_aniwave::scrape_anime;
use serenity::builder::CreateMessage;
use serenity::{model::channel::Message, prelude::*};

pub async fn exec(ctx: &Context, msg: &Message, nine_anime_id: &str, latest_episode: &str) {
    let content: String = format!("Let me try to scrape that from aniwave.to!");
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("version: error sending message: {:?}", why);
    }

    let latest_ep: i32 = latest_episode.parse::<i32>().unwrap();
    let anime: scrape_aniwave::Anime = scrape_anime(nine_anime_id).await.unwrap();

    let entry: models::NewWatchListEntry = models::NewWatchListEntry {
        name: &anime.name,
        description: &anime.description,
        nine_anime_id,
        post_img_url: &anime.poster_img_url,
        latest_episode: latest_ep,
        total_episodes: anime.total_episodes as i32,
    };

    let data: tokio::sync::RwLockReadGuard<TypeMap> = ctx.data.read().await;
    let pool: &Pool<ConnectionManager<PgConnection>> = data.get::<PgPool>().unwrap();
    let connection: &mut PooledConnection<ConnectionManager<PgConnection>> =
        &mut pool.get().unwrap();

    pg_client::add_watchlist_entry(connection, &entry);

    let mut message = CreateMessage::default();
    message.embed(|e| {
        e.colour(0x800080)
            .thumbnail(anime.poster_img_url)
            .title(format!("\"{}\" added to watchlist!", &anime.name))
            .description(&anime.description)
    });

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            *m = message;
            m
        })
        .await
    {
        println!("watchlist_add: error sending message: {:?}", why);
    }
}
