extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;
use std::env;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello\n")))
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler);
    let addr = format!("{}:{}", 
                       env::var("HOST").unwrap(), 
                       env::var("PORT").unwrap());
    Iron::new(router).http(&addr[..]).unwrap();
}
