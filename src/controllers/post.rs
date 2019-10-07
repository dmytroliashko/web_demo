use hyper::{Response, Body, StatusCode};
use gotham::state::{State, FromState};
use gotham::helpers::http::response::create_response;
use web_demo::establish_connection;
use web_demo::schema::posts;
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
pub struct QueryStringExtractor {
    id: i32,
}

pub fn get(mut state: State) -> (State, Response<Body>) {
    use web_demo::schema::posts::dsl::posts;

    let connection = establish_connection();

    let res = {
        let query_params = QueryStringExtractor::take_from(&mut state);

        let post: Post = posts
            .find(query_params.id)
            .first(&connection)
            .unwrap_or_else(|_| { panic!("Unable to find post with ID {}", query_params.id)});

        let body = serde_json::to_string(&post).unwrap();

        create_response(
            &state,
            StatusCode::from_u16(200).unwrap(),
            mime::APPLICATION_JSON,
            body
        )
    };

    (state, res)
}