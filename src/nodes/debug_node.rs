use crate::nodes::node::{FlowNode,Node,RunResult,MessageType,Runnable};

#[derive(Clone)]
pub struct DebugNode {
    pub message: String,
    pub output_keys: Vec<String>,
}

impl FlowNode for Node<DebugNode> {
    fn clone_dyn(&self) -> Box<dyn FlowNode> {
        Box::new(self.clone())
    }
}

impl Clone for Node<DebugNode> {
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


impl Runnable for Node<DebugNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        println!("running debug node");

        if let Some(data) = self.data.clone() {
            if let Some(some_payload) = payload {
                println!("payload:\n{{");

                for output_key in data.output_keys.clone() {
                    if let Some(value) = some_payload.get(&output_key) {
                        println!("\t{}:{}", output_key, value);
                    }
                }
                println!("}}");
            }
        }

        Ok(true)
    }
}