use crate::PgPool;
use pg_client::{models, ConnectionManager, PgConnection, Pool, PooledConnection};
use serenity::{model::channel::Message, prelude::*, utils::MessageBuilder};

pub async fn exec(ctx: &Context, msg: &Message) {
    let data: tokio::sync::RwLockReadGuard<TypeMap> = ctx.data.read().await;
    let pool: &Pool<ConnectionManager<PgConnection>> = data.get::<PgPool>().unwrap();
    let connection: &mut PooledConnection<ConnectionManager<PgConnection>> =
        &mut pool.get().unwrap();

    let results: Vec<models::WatchList> = pg_client::get_watchlist(connection);

    let mut list: String = String::from("");
    for entry in results {
        list.push_str(&format!(
            "- {} (ep.{}/{}) ({})\n",
            &entry.name, entry.latest_episode, entry.total_episodes, &entry.nine_anime_id
        ));
    }

    let content: String = MessageBuilder::new()
        .push_line("Here are the shows currently on your watchlist!")
        .push_codeblock(list, Some("bash"))
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("watchlist_list: error sending message: {:?}", why);
    }
}
