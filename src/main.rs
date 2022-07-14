use http::method::Method;
use http::request::Request;
use server::Server;

fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let delete = Method::DELETE;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run()
}

pub mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Server { addr }
        }

        pub fn run(self) {
            println!("Server is listening on {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            POST,
            DELETE,
            PUT,
            PATCH,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
        }
    }
}
