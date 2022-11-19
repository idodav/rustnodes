use std::collections::HashMap;

use uuid::Uuid;

type MessageType = Option<HashMap<String, String>>;

type RunResult = Result<bool, bool>;

trait BasicNode {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn print_outputs(&self);
    fn print(&self);
}

trait Runnable {
    fn run(&self, payload: MessageType) -> RunResult;
}

trait FlowNode: BasicNode + Runnable {
    fn clone_dyn(&self) -> Box<dyn FlowNode>;
}

struct Node<T> {
    id: String,
    name: String,
    outputs: Vec<Box<dyn FlowNode>>,
    data: Option<T>,
}

#[derive(Clone)]
struct DebugNode {
    message: String,
    output_keys: Vec<String>,
}

#[derive(Clone)]
struct InjectNode {
    payload: MessageType,
}

impl<T> Node<T> {
    fn new(name: String) -> Node<T> {
        Node {
            id: Uuid::new_v4().to_string(),
            name: name,
            outputs: vec![],
            data: None,
        }
    }

    fn add_output(&mut self, node: Box<dyn FlowNode>) {
        self.outputs.push(node);
    }
}

impl<T> BasicNode for Node<T> {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn print(&self) {
        println!("id: {}", self.get_id());
        println!("name: {}", self.get_name());
    }

    fn print_outputs(&self) {
        let outputs_clone = self.outputs.as_slice();
        println!(
            "id:{}\nname:{}\noutputs:\n[",
            self.get_id(),
            self.get_name()
        );
        for output in outputs_clone.into_iter() {
            println!(
                "\t{{\n\t\tid:{}\n\t\tname:{}\n\t}},",
                output.get_id(),
                output.get_name()
            )
        }
        println!("]")
    }
}

impl FlowNode for Node<InjectNode> {
    fn clone_dyn(&self) -> Box<dyn FlowNode> {
        Box::new(self.clone())
    }
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

impl Runnable for Node<DebugNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        println!("running debug node");
        println!("payload:\n{{");

        if let Some(data) = self.data.clone() {
            if let Some(some_payload) = payload {
                for output_key in data.output_keys.clone() {
                    if let Some(value) = some_payload.get(&output_key) {
                        println!("\t{}:{}", output_key, value);
                    }
                }
            }
        }
        println!("}}");

        Ok(true)
    }
}

impl Runnable for Node<InjectNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        self.print();

        println!("running inject node");

        let outputs = self.outputs.as_slice();

        for output in outputs {
            output.run(payload.clone());
        }

        Ok(true)
    }
}

fn main() {
    let mut debug_node: Node<DebugNode> = Node::new("debug".to_string());
    let mut inject_node: Node<InjectNode> = Node::new("inject".to_string());

    debug_node.data = Some(DebugNode {
        message: "hello".to_string(),
        output_keys: vec!["key1".to_string()],
    });

    let mut inject_data = HashMap::new();

    inject_data.insert("key1".to_string(), "some data".to_string());

    inject_node.data = Some(InjectNode {
        payload: Some(inject_data),
    });

    inject_node.add_output(Box::new(debug_node));
    inject_node.run(None);
}
