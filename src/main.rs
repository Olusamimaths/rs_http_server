#![allow(dead_code)]

use std::env;
use server::Server;
use request_handler::RequestHandler;

mod server;
mod http;
mod request_handler;

fn main() {
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequestHandler::new(public_path));
}

