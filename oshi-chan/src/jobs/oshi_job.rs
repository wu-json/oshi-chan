use pg_client::{ConnectionManager, PgConnection, Pool};
use serenity::async_trait;
use serenity::http::Http;
use std::sync::Arc;
use tokio_cron_scheduler::Job;

#[async_trait]
pub trait OshiJob {
    async fn exec(http: &Arc<Http>, pool: &Pool<ConnectionManager<PgConnection>>) -> ();
    fn make_job(http: Arc<Http>, pool: Pool<ConnectionManager<PgConnection>>) -> Job;
}
