use super::schema::posts;
use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}