mod commands;
mod environment;
mod handler;
mod jobs;

use environment::{Environment, EnvironmentTrait};
use pg_client::{ConnectionManager, PgConnection, Pool};
use serenity::{framework::standard::StandardFramework, prelude::*};
use tokio_cron_scheduler::{Job, JobScheduler, JobToRun};

pub struct PgPool;

impl serenity::prelude::TypeMapKey for PgPool {
    type Value = Pool<ConnectionManager<PgConnection>>;
}

#[tokio::main]
async fn main() {
    let oshi_env: String = Environment::init();
    println!("Starting oshi-chan in {oshi_env} environment");

    let framework: StandardFramework = StandardFramework::new();
    let token: String = Environment::get_discord_token();

    let intents: GatewayIntents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let pool: Pool<ConnectionManager<PgConnection>> =
        pg_client::create_connection_pool(&Environment::get_database_url());
    {
        let connection: &mut pg_client::PooledConnection<ConnectionManager<PgConnection>> =
            &mut pool.get().unwrap();
        pg_client::migrate(connection);
    };

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating serenity client");

    {
        let mut data: tokio::sync::RwLockWriteGuard<TypeMap> = client.data.write().await;
        data.insert::<PgPool>(pool);
    }

    let mut sched = JobScheduler::new().await.unwrap();

    sched.add(
        Job::new_async("1/7 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                println!("I run async every 7 seconds");

                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                    _ => println!("Could not get next tick for 7s job"),
                }
            })
        }).unwrap()
    ).await.unwrap();

    sched.start().await.unwrap();

    if let Err(why) = client.start().await {
        println!("Serenity client error: {:?}", why);
    }
}
