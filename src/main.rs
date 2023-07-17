use http::request::Request;
use server::Server;
fn main() {
    println!("Hello, world!");

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Server { addr }
        }

        pub fn run(self) {
            print!("Listening on {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }
    mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}
