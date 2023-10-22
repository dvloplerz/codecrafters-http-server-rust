// Uncomment this block to pass the first stage
use std::{
    net::TcpListener,
    io::{Read, Write},
};

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer: [u8; 1024] = [0; 1024];
                println!("accepted new connection");
                let _ = stream.read(&mut buffer);
                let to_str = std::str::from_utf8(&buffer).unwrap();
                let mut sp_str = to_str.split_whitespace();
                let method = sp_str.next().unwrap();
                let path = sp_str.next().unwrap();
                if path != "/" {
                    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")
                          .unwrap();
                } else {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                }
                dbg!(&method);
                dbg!(&path);
            }
            Err(e) => println!("error: {}", e),
        };
    }
}
