// Cool. The standard library includes functionality for web development. I'm kinda shocked.
use std::{
    fs,
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
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // We just need the first line.

    // Response for GET requests.
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
    
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        // println!("Request: {:#?}", http_request);
    
        // Without giving the argument as a byte slice the program won't compile, which explains why we use "as_bytes" method.
        // In other words write_all is expecting that the argument is type of byte. 
        // And once again, unwrap is used to easily manage success/failure state.
        stream.write_all(response.as_bytes()).unwrap();
    }

    // Response to other requests. Cannot be GET request.
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}