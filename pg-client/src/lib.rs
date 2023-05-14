use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn connect(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
}
