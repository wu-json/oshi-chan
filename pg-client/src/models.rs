use crate::schema::watchlist;
use diesel::prelude::*;
use diesel::sql_types::{Timestamp};

#[derive(Queryable)]
pub struct WatchList {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub nine_anime_id: String,
    pub post_img_url: String,
    pub latest_episode: i32,
    pub total_episodes: i32,
}

#[derive(Insertable)]
#[diesel(table_name = watchlist)]
pub struct NewWatchListEntry<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub nine_anime_id: &'a str,
    pub post_img_url: &'a str,
    pub latest_episode: i32,
    pub total_episodes: i32,
}
