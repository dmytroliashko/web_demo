extern crate gotham;
extern crate hyper;
extern crate mime;

use crate::controllers::*;
use crate::controllers::post::{PostIdExtractor};
use gotham::router::Router;
use gotham::router::builder::{build_router, DrawRoutes, DefineSingleRoute};
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::new_pipeline;
use gotham_middleware_diesel::DieselMiddleware;
use crate::repositories::Repo;

pub fn router(repo: Repo) -> Router {
    let (chain, pipeline) =
        single_pipeline(new_pipeline().add(DieselMiddleware::new(repo)).build());

    build_router(chain, pipeline, |route| {
        route.get("/").to(index);
        route.get("/post").to(post::index);
        route.get("/post/:id").with_path_extractor::<PostIdExtractor>().to(post::get);
        route.post("/post").to(post::post)
    })
}