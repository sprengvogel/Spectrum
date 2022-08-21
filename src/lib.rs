#[macro_use]
extern crate diesel;

extern crate dotenv;

pub mod error;
pub mod models;
pub mod schema;

use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};
use dotenv::dotenv;
use std::env;

use models::NewFileEntry;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_file_entry(
    conn: &SqliteConnection,
    filename: &str,
    file_last_modified: &NaiveDateTime,
) -> Result<usize, Error> {
    use schema::fileentries;

    let new_file_entry = NewFileEntry {
        filename,
        file_last_modified,
    };

    Ok(diesel::insert_into(fileentries::table)
        .values(&new_file_entry)
        .execute(conn)?)
}
