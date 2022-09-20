use std::env;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let connection_string = env::var("DATABASE_URL").expect(
        "DATABASE_URL must be set"
    );

    PgConnection::establish(&connection_string).expect(&format!(
        "Error connecting to {}", connection_string
    ))
}