use crate::environment::{Environment, EnvironmentTrait};
use pg_client::{models, ConnectionManager, PgConnection, Pool, PooledConnection};
use scrape_9anime::is_episode_out;
use serenity::builder::CreateMessage;
use serenity::http::Http;
use std::sync::Arc;

pub async fn exec(http: &Arc<Http>, pool: &Pool<ConnectionManager<PgConnection>> ) {
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

    let channel_id = Environment::get_oshi_general_channel_id();
    for anime in new_releases {
        let mut message = CreateMessage::default();
        message.embed(|e| {
            e.colour(0x800080)
                .thumbnail(anime.post_img_url)
                .title(format!(
                    "\"{}\" episode {} is out!",
                    &anime.name,
                    anime.latest_episode + 1
                ))
                .description(format!(
                    "Check it out at https://9anime.to/watch/{}/ep-{}",
                    anime.nine_anime_id,
                    anime.latest_episode + 1
                ))
        });

        if let Err(why) = channel_id
            .send_message(&http, |m| {
                *m = message;
                m
            })
            .await
        {
            println!("check_for_new_releases: error sending message: {:?}", why);
        }
    }
}
