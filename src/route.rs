extern crate gotham;
extern crate hyper;
extern crate mime;

use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute};
use crate::controllers::*;
use hyper::{Method};

pub fn router() -> Router {
    build_simple_router(|route| {
        route.request(vec![Method::GET], "/post").to(post::index);
        route.get("/").to(index);
    })
}