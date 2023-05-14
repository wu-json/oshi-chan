use crate::PgPool;
use pg_client::{models, ConnectionManager, PgConnection, Pool, PooledConnection};
use serenity::{model::channel::Message, prelude::*};

pub async fn exec(ctx: &Context, msg: &Message) {
    let data: tokio::sync::RwLockReadGuard<TypeMap> = ctx.data.read().await;
    let pool: &Pool<ConnectionManager<PgConnection>> = data.get::<PgPool>().unwrap();
    let connection: &mut PooledConnection<ConnectionManager<PgConnection>> =
        &mut pool.get().unwrap();

}
