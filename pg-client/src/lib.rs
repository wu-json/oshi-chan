use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn connect(database_url: &str) -> PgConnection {
    let connection: PgConnection = PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url));
    println!("Successfully connected to Postgres");
    connection
}

pub fn migrate(connection: &mut impl MigrationHarness<diesel::pg::Pg>) {
    connection.run_pending_migrations(MIGRATIONS).expect("Migration failed");
}
