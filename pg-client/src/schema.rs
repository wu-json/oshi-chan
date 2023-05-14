// @generated automatically by Diesel CLI.

diesel::table! {
    watchlist (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        nine_anime_id -> Varchar,
        post_img_url -> Varchar,
        latest_episode -> Int4,
        total_episodes -> Int4,
        created_on -> Timestamp,
    }
}
