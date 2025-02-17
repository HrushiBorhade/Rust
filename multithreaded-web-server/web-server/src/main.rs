use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&stream);
    let http_req: Vec<_> = buff_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("HTTP request : {http_req:#?}");
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
