use std::vec;

use uuid::Uuid;

use crate::nodes::node::{FlowNode, MessageType, Node, RunResult, Runnable};

#[derive(Clone)]
pub struct DebugNode {
    pub message: String,
    pub output_keys: Vec<String>,
}

impl Node<DebugNode> {
    pub fn new(name: String, data: Option<DebugNode>) -> Node<DebugNode> {
        Node {
            id: Uuid::new_v4().to_string(),
            name: name,
            outputs: vec![],
            data: data,
        }
    }
}

impl FlowNode for Node<DebugNode> {
    fn add_output(&mut self, output_node: Box<dyn FlowNode>) {
        self.outputs.push(output_node);
    }
}

impl Node<DebugNode> {
    fn getMessage(&self) -> String {
        if let Some(data) = self.data.clone() {
            data.message.clone()
        } else {
            "".to_string()
        }
    }
}

impl Runnable for Node<DebugNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        println!("message: {}", self.getMessage());

        if let Some(data) = self.data.clone() {
            if let Some(some_payload) = payload {
                println!("payload:\n{{");

                for output_key in data.output_keys.clone() {
                    if let Some(value) = some_payload.get(&output_key) {
                        println!("\t{}:{:?}", output_key, value);
                    }
                }
                println!("}}");
            }
        }

        Ok(true)
    }
}
