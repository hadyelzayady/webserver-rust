use core::time::Duration;
use std::fs;
use std::io::prelude::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = webserver::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    // wait to ensure all the bytes are written to the connection
    stream.flush().unwrap();
}
