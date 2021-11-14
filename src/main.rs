mod engine;
use engine::nodes::*;
use engine::Engine;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let f = engine::Flow {
        id: 123,
        name: "flow1".to_string(),
        nodes: vec![],
    };

    let engine = Engine {
        flows: vec![f],
        port: "7878".to_string(),
        host: "localhost".to_string(),
    };
    
    engine.start_api_server();
}
