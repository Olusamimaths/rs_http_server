use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct RequestHandler;

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Welcome again </h1>".to_string()))
    }
}
