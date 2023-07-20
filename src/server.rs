use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        print!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    handle_ok_request(stream, handler);
                }
                Err(error) => println!("Failed to establish connection: {}", error),
            }
        }
    }
}

fn handle_ok_request(mut stream: std::net::TcpStream, mut handler: impl Handler) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Receieved request: {}", String::from_utf8_lossy(&buffer));

            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(e) = response.send(&mut stream) {
                println!("Failed to send response: {}", e);
            }
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}
