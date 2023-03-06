use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_conection(stream);
    }
}

fn handle_conection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_read
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request is {:?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n {contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
