extern crate gotham;
extern crate hyper;
extern crate mime;

#[macro_use]
extern crate gotham_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod route;
mod controllers;

use gotham::state::State;
use crate::route::router;

const HELLO_WORLD: &str = "Hello world!";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
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
