use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Server is listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf))
                        }
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                }
                Err(err) => println!("Failed to stablish a connection: {}", err),
            }
        }
    }
}
