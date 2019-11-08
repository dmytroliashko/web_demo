use hyper::{Response, Body, StatusCode};
use gotham::state::{State, FromState};
use gotham::helpers::http::response::create_response;
use web_demo::establish_connection;
use web_demo::models::*;
use diesel::prelude::*;
use gotham::handler::{HandlerFuture, HandlerError, IntoHandlerError};
use futures::{future, Future, Stream};
use std::str::from_utf8;
use web_demo::schema::posts;
use crate::repositories::Repo;

pub fn index(state: State) -> (State, Response<Body>) {
    use web_demo::schema::posts::dsl::posts;

    let connection = establish_connection();
    let results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let body = serde_json::to_string(&results).unwrap();

    let res = {
        create_response(
            &state,
            StatusCode::from_u16(200).unwrap(),
            mime::APPLICATION_JSON,
            body
        )
    };

    (state, res)
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct PostIdExtractor {
    id: i32,
}

pub fn get(state: State) -> (State, Response<Body>) {
    use web_demo::schema::posts::dsl::posts;

    let connection = establish_connection();

    let res = {
        let post_id = PostIdExtractor::borrow_from(&state);
        let mut status_code = 200;
        let mut body = String::new();
        let mut mime_type = mime::APPLICATION_JSON;

        let post: Post = posts
            .find(post_id.id)
            .first(&connection)
            .unwrap_or_else(|_| {
                status_code = 404;
                body = String::from(format!("Post with ID {} is not found." , post_id.id));
                mime_type = mime::TEXT_HTML;

                Post {id: 0, title: "".to_string(), published: false, body: "".to_string() }
            });

        if post.id > 0 {
            body = serde_json::to_string(&post).unwrap();
        }

        create_response(
            &state,
            StatusCode::from_u16(200).unwrap(),
            mime_type,
            body
        )
    };

    (state, res)
}



#[derive(Serialize)]
struct RowsUpdated {
    rows: usize,
}

pub fn post(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let f = extract_json::<NewPost>(&mut state)
        .and_then(move |post| {
            repo.run(move |conn| {
                diesel::insert_into(posts::table)
                    .values(&post)
                    .execute(&conn)
            })
                .map_err(|e| e.into_handler_error())
        })
        .then(|result| match result {
            Ok(rows) => {
                let body  = serde_json::to_string(&RowsUpdated { rows })
                    .expect("Failed to serialize to json");
                let res =
                    create_response(&state, StatusCode::from_u16(201).unwrap(), mime::APPLICATION_JSON, body);
                future::ok((state, res))
            }
            Err(e) => {
                future::err((state, e))
            }
        });
    Box::new(f)
}

fn bad_request<E>(e: E) -> HandlerError
    where
        E: std::error::Error + Send + 'static,
{
    e.into_handler_error().with_status(StatusCode::BAD_REQUEST)
}

fn extract_json<T>(state: &mut State) -> impl Future<Item = T, Error = HandlerError>
    where
        T: serde::de::DeserializeOwned,
{
    Body::take_from(state)
        .concat2()
        .map_err(bad_request)
        .and_then(|body| {
            let b = body.to_vec();
            from_utf8(&b)
                .map_err(bad_request)
                .and_then(|s| serde_json::from_str::<T>(s).map_err(bad_request))
        })
}