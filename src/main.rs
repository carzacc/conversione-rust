extern crate iron;
extern crate router;
mod html;

use router::Router;
use iron::request::*;
use iron::Response;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

fn main() {
    let mut router = Router::new();

    router.get("/", invia_form, "radice"); // Molto simile alla sintassi che si usa con Express.js
    // router.post("/conversione", invia_conversione, "MCD");

    println!("Server avviato su http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn invia_form(_request: &mut Request) -> IronResult<Response> {
    let mut r = Response::new();
    let mime: Mime = "text/html".parse().unwrap();
    r.set_mut(status::Ok);
    r.set_mut(mime);
    r.set_mut(html::html::form);

    Ok(r)
}

// fn invia_conversione(_: Request) -> IronResult<Response> {

