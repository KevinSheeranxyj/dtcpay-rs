use std::fmt::format;
use diesel::PgConnection;


use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use sea_orm::sqlx::PgConnection;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))

}
