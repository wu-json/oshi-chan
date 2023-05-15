use crate::PgPool;
use pg_client::{models, ConnectionManager, PgConnection, Pool, PooledConnection};
use scrape_9anime::is_episode_out;
use serenity::{model::channel::Message, prelude::*, utils::MessageBuilder};

pub async fn exec(ctx: &Context) {
    let data: tokio::sync::RwLockReadGuard<TypeMap> = ctx.data.read().await;
    let pool: &Pool<ConnectionManager<PgConnection>> = data.get::<PgPool>().unwrap();
    let connection: &mut PooledConnection<ConnectionManager<PgConnection>> =
        &mut pool.get().unwrap();

    let results: Vec<models::WatchList> = pg_client::get_watchlist(connection);
    let mut new_releases: Vec<models::WatchList> = Vec::new();

    for anime in results {
        // series is finished so we exit
        if anime.latest_episode + 1 > anime.total_episodes {
            continue;
        }

        let new_episode: u32 = (anime.latest_episode + 1) as u32;
        let new_episode_out = is_episode_out(&anime.nine_anime_id, new_episode)
            .await
            .unwrap();

        if new_episode_out {
            pg_client::update_watchlist_entry(connection, &anime.nine_anime_id, new_episode as i32);
            new_releases.push(anime);
        }
    }
}
