mod commands;
mod environment;
mod handler;
mod jobs;

use environment::{Environment, EnvironmentTrait};
use pg_client::{ConnectionManager, PgConnection, Pool};
use serenity::{framework::standard::StandardFramework, prelude::*};
use tokio_cron_scheduler::JobScheduler;

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
    let job_factories = [jobs::check_for_new_releases::make_job];
    for make_job in job_factories {
        let http_clone = client.cache_and_http.http.clone();
        let pool_clone = pool.clone();
        sched.add(make_job(http_clone, pool_clone)).await.unwrap();
    }

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
