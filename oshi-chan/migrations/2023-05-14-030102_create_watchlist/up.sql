CREATE TABLE watchlist (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    nine_anime_id VARCHAR NOT NULL,
    post_img_url VARCHAR NOT NULL,
    latest_episode INT NOT NULL,
    total_episodes INT NOT NULL,
    created_on TIMESTAMP NOT NULL,
);

CREATE INDEX ix_watchlist_nine_anime_id ON watchlist(nine_anime_id);
CREATE INDEX ix_watchlist_latest_episode ON watchlist(latest_episode);
CREATE INDEX ix_watchlist_total_episodes ON watchlist(total_episodes);