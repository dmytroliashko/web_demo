use gotham::state::State;
use hyper::{Response, Body, StatusCode};
use gotham::helpers::http::response::create_response;

pub mod post;

pub fn index(mut state: State) -> (State, Response<Body>) {
    let body = "<html><body><h1>Hello Gotham page</h1></body></html>";

    let res = {
        create_response(
            &state,
            StatusCode::from_u16(200).unwrap(),
            mime::TEXT_HTML_UTF_8,
            body
        )
    };

    (state, res)
}
