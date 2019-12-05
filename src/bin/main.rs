use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on: http://127.0.0.1:7878/");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
           handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let(status, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 OK\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!("{} {}", status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
