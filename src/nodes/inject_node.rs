use crate::nodes::node::{MessageType, Node, FlowNode, Runnable, RunResult, BasicNode};

#[derive(Clone)]
pub struct InjectNode {
    pub payload: MessageType,
}


impl FlowNode for Node<InjectNode> {
    fn clone_dyn(&self) -> Box<dyn FlowNode> {
        Box::new(self.clone())
    }
}



impl Clone for Node<InjectNode> {
    fn clone(&self) -> Self {
        let mut vec = vec![];

        for item in self.outputs.as_slice() {
            vec.push(item.clone_dyn());
        }

        Node {
            id: self.id.clone(),
            name: self.name.clone(),
            outputs: vec,
            data: self.data.clone(),
        }
    }
}

impl Runnable for Node<InjectNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        self.print();

        println!("running inject node");

        let outputs = self.outputs.as_slice();

        if let Some(some_payload) = self.data.clone() {
            for output in outputs {
                output.run(some_payload.payload.clone());
            }
        }

        Ok(true)
    }
}