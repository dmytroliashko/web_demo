use hyper::{Response, Body, StatusCode};
use gotham::state::{State, FromState};
use gotham::helpers::http::response::create_response;
use web_demo::establish_connection;
use web_demo::models::*;
use diesel::prelude::*;

pub fn index(mut state: State) -> (State, Response<Body>) {
    use web_demo::schema::posts::dsl::posts;

    let connection = establish_connection();
    let results = posts
        .limit(5)
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
pub struct ProductIdExtractor {
    id: i32,
}

pub fn get(mut state: State) -> (State, Response<Body>) {
    use web_demo::schema::posts::dsl::posts;

    let connection = establish_connection();

    let res = {
        let productId = ProductIdExtractor::borrow_from(&state);
        let mut status_code = 200;
        let mut body = String::new();
        let mut mime_type = mime::APPLICATION_JSON;

        let post: Post = posts
            .find(productId.id)
            .first(&connection)
            .unwrap_or_else(|_| {
                status_code = 404;
                body = String::from(format!("Post with ID {} is not found." , productId.id));
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