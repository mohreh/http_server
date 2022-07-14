use std::net::TcpListener;

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
                Ok((stram, _)) => {
                    println!("OK")
                }
                Err(err) => println!("Failed to stablish a connection: {}", err),
            }
        }
    }
}
