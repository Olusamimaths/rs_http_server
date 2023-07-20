use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>HomePage</h1>".to_string())),
                "/posts" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>This is our page</h1>".to_string()),
                ),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
