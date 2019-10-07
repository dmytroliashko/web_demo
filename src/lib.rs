#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::MysqlConnection;
use models::{Post, NewPost};

pub fn establish_connection() -> MysqlConnection {
    dotenv();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts::dsl::{id, posts};

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post.");

    posts.order(id.desc()).first(conn).unwrap()
}