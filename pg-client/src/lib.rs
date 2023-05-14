pub mod models;
pub mod schema;

pub use diesel::pg::PgConnection;
pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::NewWatchListEntry;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_connection_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build Postgres connection pool")
}

pub fn migrate(connection: &mut impl MigrationHarness<diesel::pg::Pg>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Migration failed");
    println!("Successfully completed Postgres migrations")
}

pub fn add_watchlist_entry(connection: &mut PgConnection, entry: &NewWatchListEntry) {
    diesel::insert_into(schema::watchlist::table)
        .values(entry)
        .execute(connection)
        .unwrap();
}
