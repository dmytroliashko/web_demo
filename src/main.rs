extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate dotenv;
extern crate serde;

#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate serde_derive;

mod route;
mod controllers;

use gotham::state::State;
use crate::route::router;
use gotham_middleware_diesel::Repo;
use std::env;
use dotenv::dotenv;

const HELLO_WORLD: &str = "Hello world!";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    gotham::start(addr, router(Repo::new(&database_url)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::testServer;
    use hyper::StatusCode;
    use gotham::tls::test::TestServer;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
