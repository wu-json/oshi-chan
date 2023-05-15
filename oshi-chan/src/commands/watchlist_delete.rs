use crate::PgPool;
use pg_client::{ConnectionManager, PgConnection, Pool, PooledConnection};
use serenity::{model::channel::Message, prelude::*};

pub async fn exec(ctx: &Context, msg: &Message, nine_anime_id: &str) {
    let data: tokio::sync::RwLockReadGuard<TypeMap> = ctx.data.read().await;
    let pool: &Pool<ConnectionManager<PgConnection>> = data.get::<PgPool>().unwrap();
    let connection: &mut PooledConnection<ConnectionManager<PgConnection>> =
        &mut pool.get().unwrap();

    pg_client::delete_watchlist_entry(connection, nine_anime_id);

    let content: String = format!("I just deleted {} from the watchlist!", nine_anime_id);
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("version: error sending message: {:?}", why);
    }
}
