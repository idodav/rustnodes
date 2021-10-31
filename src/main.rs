mod engine;
use engine::nodes::*;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let c = Nodes::Debug(Debug {
        id: 1234,
        name: "second node".to_string(),
        out_nodes: None,
    });

    let b = Nodes::Debug(Debug {
        id: 1234,
        name: "second node".to_string(),
        out_nodes: None,
    });

    let a = Nodes::Inject(Inject {
        id: 1234,
        name: "first node".to_string(),
        message: "injected first message".to_string(),
        out_nodes: Some(vec![b, c]),
    });

    a.to_inject().inject();

    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let received = std::str::from_utf8(&buffer).expect("valid utf8");
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer);
    println!("{} {}", req.method.unwrap(),req.path.unwrap());
    
    let response = "HTTP/1.1 200 OK\r\n\r\n as asjkasjh".to_string();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}