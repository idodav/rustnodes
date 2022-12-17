use std::vec;

use uuid::Uuid;

use crate::nodes::node::{FlowNode, MessageType, Node, RunResult, Runnable};

#[derive(Clone)]
pub struct SplitNode {
}

impl Node<SplitNode> {
   pub fn new(name:String) -> Node<SplitNode>{
        Node {
            id:Uuid::new_v4().to_string(),
            name: name,
            outputs: vec![],
            data:None
        }
   }
}

impl FlowNode for Node<SplitNode> {
    fn add_output(&mut self, output_node: Box<dyn FlowNode>) {
        self.outputs.push(output_node);
    }
}

impl Node<SplitNode> {

}

impl Runnable for Node<SplitNode> {
    fn run(&self, payload: MessageType) -> RunResult {

        Ok(true)
    }
}
