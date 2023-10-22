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
                println!("accepted new connection");
                match stream.read(&mut [0; 128]) {
                    Ok(r) => {
                        println!("{r}");
                        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(e) => println!("error: {}", e),
        };
    }
}
