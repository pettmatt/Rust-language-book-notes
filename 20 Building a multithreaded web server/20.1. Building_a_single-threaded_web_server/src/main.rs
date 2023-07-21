// Cool. The standard library includes functionality for web development. I'm kinda shocked.
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Remember that HTTP protocol runs on top of TCP, so we're using more barebones protocol, but it also gives us more control if we need it.
    // Creating new instance of a listener with a specific port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // And here we're listening the port specified above and processing the requests.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
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
}