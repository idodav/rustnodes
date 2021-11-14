pub mod nodes {
    include!("nodes.rs");
}
pub use nodes::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Flow {
    pub id: i32,
    pub name: String,
    pub nodes: Vec<Nodes>,
}

pub struct Message {
    id: i32,
    payload: String,
}

pub struct Engine {
    pub flows: Vec<Flow>,
    pub port: String,
    pub host: String,
}

impl Engine {
    pub fn start_api_server(&self) {
        let port = &self.port;
        let host = &self.host;
        let bindString = format!("{}:{}", host, port);

        // Listen for incoming TCP connections on localhost port 7878
        let listener = TcpListener::bind(&bindString).unwrap();
        println!("Server is listening on {}", &bindString);

        // Block forever, handling each request that arrives at this IP address
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            &self.handle_connection(stream);
        }
    }

    pub fn add_flow(&mut self, flow: Flow) -> usize {
        &self.flows.push(flow);
        (&self.flows).len()
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        // Read the first 32768 bytes of data from the stream
        let mut buffer = [0; 32768];
        stream.read(&mut buffer).unwrap();
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(&buffer);
        self.handle_request(&stream, buffer, req.method.unwrap(), req.path.unwrap());
    }

    fn handle_request(&self, stream: &TcpStream, buffer: [u8; 32768], method: &str, path: &str) {
        println!("{} {}", method, path);
        let received = std::str::from_utf8(&buffer).expect("valid utf8");
        let split = received.split("\r\n\r\n");
        let vec: Vec<&str> = split.collect();
        let _header = vec.get(0).unwrap();
        let body = vec.get(1).unwrap();
        match (method, path) {
            ("POST", "/addNode") => self.handle_add_node(stream, body),
            ("POST", "/addFlow") => self.handle_add_node(stream, body),
            ("GET", "/flow") => self.hadnle_get_flows(stream),
            ("GET", "/healthcheck") => self.respond_ok(stream, None),
            _ => self.respond_not_found(stream),
        };
    }

    pub fn hadnle_get_flows(&self, stream: &TcpStream) {
        let mut res: String = "".to_string();
        res.push_str("{\r\n\tflows:[\r\n\t\t");
        for flow in &self.flows {
            let fl_str = format!("{{\"{}\":\"{}\"}},", flow.id, flow.name);
            res.push_str(&fl_str);
        }
        res.push_str("\r\n\t]\r\n}");
        &self.respond_ok(stream, Some(res));
    }
    
    pub fn hadnle_add_flow(&self, stream: &TcpStream) {
        let mut res: String = "".to_string();
        res.push_str("{\r\n\tflows:[\r\n\t\t");
        for flow in &self.flows {
            let fl_str = format!("{{\"{}\":\"{}\"}},", flow.id, flow.name);
            res.push_str(&fl_str);
        }
        res.push_str("\r\n\t]\r\n}");
        &self.respond_ok(stream, Some(res));
    }

    fn handle_add_node(&self, stream: &TcpStream, body: &str) {
        println!("handle_add_node");
        let add_node_body = json::parse(&*body.replace("\u{0}", "")).unwrap();
        let id = add_node_body["id"].as_u32().unwrap();
        let name = add_node_body["name"].as_str().unwrap().to_owned();
        let new_node: Nodes = Nodes::Node(Node {
            id: id,
            name: name,
            out_nodes: None,
        });
        self.respond_ok(stream, None);
    }

    fn respond_not_found(&self, mut stream: &TcpStream) {
        let response = "HTTP/1.1 404 Not Found\r\n\r\nNot found".to_string();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn respond_ok(&self, mut stream: &TcpStream, body: Option<String>) {
        let response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        stream.write(response.as_bytes()).unwrap();
        if let Some(body_value) = body {
            stream.write(body_value.as_bytes()).unwrap();
        }
        stream.flush().unwrap();
    }
}
