use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    handle_ok_request(stream);
                }
                Err(error) => println!("Failed to establish connection: {}", error),
            }
        }
    }
}

fn handle_ok_request(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Receieved request: {}", String::from_utf8_lossy(&buffer));

            match Request::try_from(&buffer[..]) {
                Ok(request) => {
                    // dbg!(request);
                    let response = Response::new(StatusCode::Ok, Some(
                        "<h1>Welcome to My Rust Server</h1>".to_string()
                    ));
                    write!(stream, "{}", response);
                }
                Err(e) => print!("Failed to parse a request: {}", e),
            }
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}
