use std::net::TcpListener;

// Uncomment this block to pass the first stage
use request::request;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    request(listener).ok();
}

pub mod error {
    pub enum Error {
        InvalidRequest,
        InvalidProtocol,
        InvalidMethod,
        IO(String),
        Utf8(String),
    }

    impl From<std::str::Utf8Error> for Error {
        fn from(value: std::str::Utf8Error) -> Self {
            Self::Utf8(value.to_string())
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::IO(value.to_string())
        }
    }
}

pub mod method {
    use std::str::FromStr;
    use crate::error::Error;

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
                "GET" => Ok(Method::Get),
                "POST" => Ok(Method::Post),
                "PUT" => Ok(Method::Put),
                "DELETE" => Ok(Method::Delete),
                _ => Err(Error::InvalidMethod),
            }
        }
    }
}

pub mod http_status {
    use std::fmt::Display;

    #[derive(Debug)]
    pub enum HttpStatus {
        OK = 200,
        BadRequest = 400,
        NotFound = 404,
    }

    impl Display for HttpStatus {
        // add code here
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let message = match self {
                Self::OK => "200 OK",
                Self::BadRequest => "400 Bad Request",
                Self::NotFound => "404 Not Found",
            };
            write!(f, "{}", message)
        }
    }
}

pub mod response {
    use std::fmt::Display;
    use std::net::TcpStream;
    use std::io::Write;
    use crate::http_status::HttpStatus;

    #[derive(Debug)]
    pub struct Response {
        http_status: HttpStatus,
        body: Option<String>,
    }

    impl Response {
        pub fn new(http_status: HttpStatus, body: Option<String>) -> Self {
            Self { http_status, body }
        }

        pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
            let body = match &self.body {
                Some(v) => v,
                None => "",
            };
            let body_len = body.len();
            write!(stream,
                   "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                   self.http_status, body_len, body)
        }
    }

    impl Display for Response {
        // add code here
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let content = "";
            write!(f, "Response::Display:: {}", &content)
        }
    }
}

pub mod request {
    use std::{net::TcpListener, io::Read};
    use crate::{method::Method, http_status::HttpStatus};
    use crate::response::Response;
    use crate::error::Error;

    pub struct Request {
        method: Method,
    }

    impl Request {
        pub fn method(&self) -> &Method {
            &self.method
        }
    }

    pub fn request(listener: TcpListener) -> Result<(), Error> {
        let mut buffer: [u8; 1024] = [0; 1024];

        for stream in listener.incoming() {
            let mut stream = stream?;
            let _ = stream.read(&mut buffer);
            let buffers = std::str::from_utf8(&buffer)?;

            let mut headers = buffers.split("\r\n");

            let mut sp_str = headers.next().unwrap().split_whitespace();
            let _method: Method =
                sp_str.next().ok_or(Error::InvalidMethod)?.parse()?;
            let _path = sp_str.next().unwrap();

            let _host = headers.next().unwrap();

            let _user_agent = headers.next().unwrap();

            if _path == "/" {
                let _ = Response::new(HttpStatus::OK, None).send(&mut stream);
            } else if _path.contains("/echo") {
                let echo =
                    _path.split("/echo/").collect::<Vec<_>>()[1].to_string();

                let _ =
                    Response::new(HttpStatus::OK, Some(echo)).send(&mut stream);
            } else if _path == "/user-agent" {
                let ua = _user_agent.split_whitespace().collect::<Vec<_>>()[1]
                    .to_string();
                let _ =
                    Response::new(HttpStatus::OK, Some(ua)).send(&mut stream);
            } else {
                let _ =
                    Response::new(HttpStatus::NotFound, None).send(&mut stream);
            }
        }
        Ok(())
    }
}
