use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        print!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Err(error) => println!("Failed to establish connection: {}", error),

                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Receieved request: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
            }
        }
    }
}
