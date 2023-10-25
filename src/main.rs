// Uncomment this block to pass the first stage
// use std::net::TcpListener;
use std::net::TcpListener;

use crate::handler::handle_connection;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221")
        .expect("Cannot bind to Address or Port.");

    for stream in listener.incoming() {
        let stream = stream.expect("TcpStream Error.");

        handle_connection(stream);
    }
}

pub mod handler {
    use std::{
        io::{prelude::*, BufReader},
        net::TcpStream,
    };
    use crate::{http_status::HttpStatus, response::Response};

    pub fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> =
            buf_reader.lines()
                      .map(|result| result.unwrap())
                      .take_while(|line| !line.is_empty())
                      .collect();

        let request_clone = http_request.clone();

        let _request_info =
            extract_request(request_clone.first().unwrap().as_str());

        let _method = _request_info.0;
        let path = _request_info.1;

        match extract_path(path)[0] {
            "" => {
                Response::new(
                    HttpStatus::OK,
                    Some("This is home!".to_string()),
                )
                .send(&mut stream);
            }
            "echo" => {
                let response_body =
                    extract_path(path)[1..].join("/").to_string();
                Response::new(HttpStatus::OK, Some(response_body))
                    .send(&mut stream);
            }
            "user-agent" => {
                let user_agent = request_clone
                    .iter()
                    .filter(|v| v.contains("User-Agent"))
                    .collect::<Vec<_>>()[0]
                    .as_str()
                    .split_whitespace()
                    .collect::<Vec<_>>()[1]
                    .to_string();
                Response::new(HttpStatus::OK, Some(user_agent))
                    .send(&mut stream);
            }
            _ => Response::new(HttpStatus::NotFound, None).send(&mut stream),
        }
    }

    pub fn extract_request(request_info: &str) -> (&str, &str) {
        let mut request = request_info.split_whitespace();

        let method = request.next().unwrap_or("GET");
        let path = request.next().unwrap_or("/");

        (method, path)
    }

    pub fn extract_path(path: &str) -> Vec<&str> {
        let path = path.split('/').collect::<Vec<_>>();
        path[1..].to_vec()
    }
}

mod http_status {
    pub enum HttpStatus {
        OK = 200,
        BadRequest = 400,
        NotFound = 404,
    }

    impl std::fmt::Display for HttpStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let message = match &self {
                Self::OK => "200 OK",
                Self::BadRequest => "400 BadRequest",
                Self::NotFound => "404 Not Found",
            };
            write!(f, "{message}")
        }
    }
}

pub mod response {
    use std::{net::TcpStream, io::Write};

    use crate::http_status::HttpStatus;

    pub struct Response {
        http_status: HttpStatus,
        body: Option<String>,
    }

    impl Response {
        pub fn new(http_status: HttpStatus, body: Option<String>) -> Self {
            Self { http_status, body }
        }

        pub fn send(&self, stream: &mut TcpStream) {
            let body = match &self.body {
                Some(body) => body,
                None => "",
            };

            let body_length = body.len();

            let response =
                format!("HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", self.http_status, body_length, body);

            stream.write_all(response.as_bytes())
                  .expect("Cannot Write to stream.");
        }
    }
}
