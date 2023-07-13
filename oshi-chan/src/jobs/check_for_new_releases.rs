use crate::environment::{Environment, EnvironmentTrait};
use crate::jobs::oshi_job::OshiJob;
use pg_client::{models, ConnectionManager, PgConnection, Pool};
use scrape_9anime::is_episode_out;
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::http::Http;
use std::sync::Arc;
use tokio_cron_scheduler::Job;

pub struct CheckForNewReleasesJob {}

fn get_watchlist(pool: &Pool<ConnectionManager<PgConnection>>) -> Vec<models::WatchList> {
    let connection = &mut pool.get().unwrap();
    pg_client::get_watchlist(connection)
}

#[async_trait]
impl OshiJob for CheckForNewReleasesJob {
    async fn exec(http: &Arc<Http>, pool: &Pool<ConnectionManager<PgConnection>>) -> () {
        let results: Vec<models::WatchList> = get_watchlist(pool);
        let mut new_releases: Vec<models::WatchList> = Vec::new();

        println!("Checking for new releases for {} shows", results.len());

        for anime in results {
            // series is finished so we exit
            if anime.latest_episode + 1 > anime.total_episodes {
                continue;
            }

            let new_episode: u32 = (anime.latest_episode + 1) as u32;
            let new_episode_out: bool =
                match is_episode_out(&anime.nine_anime_id, new_episode, None).await {
                    Ok(v) => v,
                    Err(err) => {
                        println!(
                            "Error occurred while checking for new releases for {}: {}",
                            &anime.nine_anime_id,
                            err.to_string()
                        );
                        false
                    }
                };

            println!(
                "{}: episode {} {}",
                anime.nine_anime_id,
                new_episode,
                if new_episode_out {
                    "is out!"
                } else {
                    "is not out"
                }
            );

            if new_episode_out {
                {
                    let connection = &mut pool.get().unwrap();
                    pg_client::update_watchlist_entry(
                        connection,
                        &anime.nine_anime_id,
                        new_episode as i32,
                    );
                }
                new_releases.push(anime);
            }
        }

        println!("Found {} new releases", new_releases.len());

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

    fn make_job(http: Arc<Http>, pool: Pool<ConnectionManager<PgConnection>>) -> Job {
        Job::new_async("0 1/15 * * * *", move |uuid, mut l| {
            let http = http.clone();
            let pool = pool.clone();
            Box::pin(async move {
                println!("New releases job started");
                CheckForNewReleasesJob::exec(&http, &pool).await;
                println!("New releases job completed");

                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Next time for new releases job is {:?}", ts),
                    _ => println!("Could not get next tick for new releases job"),
                }
            })
        })
        .unwrap()
    }
}
