use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                   return fs::read_to_string(path).ok();
                } else {
                    println!("Directory Traversal Attack attempted!!!, filepath: {}", file_path);
                    return None;
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/posts" => Response::new(
                    StatusCode::Ok,
                    self.read_file("post.html"),
                ),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
