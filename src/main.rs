use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let port: String = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: String = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n\
    Content-Type: text/html; charset=UTF-8\r\n\r\n\
    Hello world\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
