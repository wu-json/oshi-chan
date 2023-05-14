use crate::PgPool;

use pg_client::{ConnectionManager, PooledConnection, PgConnection, Pool, models};
use scrape_9anime::scrape_anime;
use serenity::{model::channel::Message, prelude::*};

pub async fn exec(ctx: &Context, msg: &Message, nine_anime_id: &str, latest_episode: &str) {
    let latest_ep: i32 = latest_episode.parse::<i32>().unwrap();
    let anime: scrape_9anime::Anime = scrape_anime(nine_anime_id).await.unwrap();

    let entry: models::NewWatchListEntry = models::NewWatchListEntry {
        name: &anime.name,
        description: &anime.description,
        nine_anime_id,
        post_img_url: &anime.poster_img_url,
        latest_episode: latest_ep,
        total_episodes: anime.total_episodes as i32
    };

    let mut data: tokio::sync::RwLockReadGuard<TypeMap> = ctx.data.read().await;
    let pool: &Pool<ConnectionManager<PgConnection>> = data.get::<PgPool>().unwrap();
    let connection: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut pool.get().unwrap();

    pg_client::add_watchlist_entry(connection, &entry);
}
