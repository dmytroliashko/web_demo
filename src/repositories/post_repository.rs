use crate::repositories::Repo;
use web_demo::models::NewPost;
use web_demo::schema::posts;
use web_demo::establish_connection;
use diesel::{MysqlConnection, RunQueryDsl};
use gotham::handler::{IntoHandlerError, HandlerError};
use futures::{Future};
use std::borrow::Borrow;

pub struct PostRepository {
    connection: MysqlConnection,
    inner: Repo
}

impl PostRepository {
    pub fn create(inner: Repo) -> Self {
        let connection = establish_connection();
        PostRepository { connection, inner }
    }

//TODO: implement add method.
//    pub fn add<F>(&self, new_post: NewPost) -> &impl Future<Item=usize, Error=HandlerError>
//        where F: Future<Item = usize>
//    {
//        let connection = self.connection.;
//        &self.inner.run(move |connection| {
//            diesel::insert_into(posts::table)
//                .values(&new_post)
//                .execute(&connection)
//                .map_err(|e| e.into_handler_error())
//        })
//    }
}