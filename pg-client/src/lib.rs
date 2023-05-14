pub mod models;
pub mod schema;

pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
pub use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_connection_pool(database_url: &str)-> Pool<ConnectionManager<PgConnection>> {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Failed to build Postgres connection pool")
}

pub fn connect(database_url: &str) -> PgConnection {
    let connection: PgConnection = PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url));
    println!("Successfully connected to Postgres");
    connection
}

pub fn migrate(connection: &mut impl MigrationHarness<diesel::pg::Pg>) {
    connection.run_pending_migrations(MIGRATIONS).expect("Migration failed");
    println!("Successfully completed Postgres migrations")
}
