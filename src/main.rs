use std::net::TcpListener;

// Uncomment this block to pass the first stage
use request::request;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    request(listener);
}

pub mod method {
    use std::str::FromStr;
    use std::io::Error;

    #[derive(Debug)]
    pub enum Method {
        Get,
        Post,
        Put,
        Delete,
    }

    impl FromStr for Method {
        type Err = Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            match value {
                "Get" => Ok(Method::Get),
                "Post" => Ok(Method::Post),
                "Put" => Ok(Method::Put),
                "Delete" => Ok(Method::Delete),
                _ => panic!("Invalid Method!"),
            }
        }
    }
}

pub mod request {
    use std::{
        net::TcpListener,
        io::{Read, Write},
    };
    use crate::method::Method;

    pub struct Request {
        method: Method,
        path: String,
    }

    impl Request {
        pub fn method(&self) -> &Method {
            &self.method
        }
    }

    pub fn request(listener: TcpListener) {
        let mut buffer: [u8; 1024] = [0; 1024];
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _ = stream.read(&mut buffer);
                    let buffers = std::str::from_utf8(&buffer).unwrap();
                    let mut sp_str = buffers.split_whitespace();
                    let _method: Method =
                        sp_str.next().unwrap().parse().unwrap();
                    let _path = sp_str.next().unwrap();
                    if _path == "/" {
                        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    }
                    if _path.contains("/echo") {
                        let echo = _path.split("/echo/").collect::<Vec<_>>()[1]
                            .to_string();

                        println!("{:?}", echo);
                        let _ = stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo.len(), echo).as_bytes());
                    } else {
                        stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")
                              .unwrap();
                    }
                }
                Err(e) => println!("error: {}", e),
            };
        }
    }
}
