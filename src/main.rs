use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1101").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let _contents = fs::read_to_string("index.html").unwrap();
    let _response = format!("HTTP/1.1 200 OK\r\n\r\n{}", _contents);

    stream.write(_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}