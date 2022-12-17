use uuid::Uuid;

use crate::nodes::node::{BasicNode, FlowNode, MessageType, Node, RunResult, Runnable};

#[derive(Clone)]
pub struct InjectNode {
    pub payload: MessageType,
}

impl<'a> FlowNode for Node<InjectNode> {
    fn add_output(&mut self, output_node: Box<dyn FlowNode>) {
        self.outputs.push(output_node);
    }
}

impl Node<InjectNode> {
    pub fn new(name: String, data: Option<InjectNode>) -> Node<InjectNode> {
        Node {
            id: Uuid::new_v4().to_string(),
            name: name,
            outputs: vec![],
            data: data,
        }
    }
}

impl<'a> Runnable for Node<InjectNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        self.print();

        println!("running inject node");

        let outputs = self.outputs.as_slice();
        println!("runnin {} nodes", outputs.len());

        if let Some(some_payload) = self.data.clone() {
            for output in outputs {
                println!("running {}", output.get_id());
                output.run(some_payload.payload.clone());
            }
        }

        Ok(true)
    }
}
