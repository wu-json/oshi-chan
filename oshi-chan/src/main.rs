mod commands;
mod environment;
mod handler;
mod jobs;

use environment::{Environment, EnvironmentTrait};
use pg_client::{ConnectionManager, PgConnection, Pool};
use serenity::{framework::standard::StandardFramework, prelude::*};
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct PgPool;

impl serenity::prelude::TypeMapKey for PgPool {
    type Value = Pool<ConnectionManager<PgConnection>>;
}

#[tokio::main]
async fn main() {
    let oshi_env = Environment::init();
    println!("Starting oshi-chan in {oshi_env} environment");

    let framework = StandardFramework::new();
    let token = Environment::get_discord_token();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let pool = pg_client::create_connection_pool(&Environment::get_database_url());
    {
        let connection = &mut pool.get().unwrap();
        pg_client::migrate(connection);
    };

    let discord_client_pool = pool.clone();
    let mut client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating serenity client");
    {
        let mut data = client.data.write().await;
        data.insert::<PgPool>(discord_client_pool);
    }

    let mut sched = JobScheduler::new().await.unwrap();
    let http = client.cache_and_http.http.clone();

    sched
        .add(
            Job::new_async("0 1/15 * * * *", move |uuid, mut l| {
                let http = http.clone();
                let pool = pool.clone();
                Box::pin(async move {
                    println!("New releases job started");
                    jobs::check_for_new_releases::exec(&http, &pool).await;
                    println!("New releases job completed");

                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => println!("Next time for new releases job is {:?}", ts),
                        _ => println!("Could not get next tick for new releases job"),
                    }
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    sched.start().await.unwrap();

    if let Err(why) = client.start().await {
        println!("Serenity client error: {:?}", why);
    }
}
