use std::collections::HashMap;

use uuid::Uuid;

pub type MessageType = Option<HashMap<String, String>>;

pub type RunResult = Result<bool, bool>;

pub trait BasicNode {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn print_outputs(&self);
    fn print(&self);
}

pub trait Runnable {
    fn run(&self, payload: MessageType) -> RunResult;
}

pub trait FlowNode: BasicNode + Runnable {
    fn clone_dyn(&self) -> Box<dyn FlowNode>;
}

pub struct Node<T> {
    pub id: String,
    pub name: String,
    pub outputs: Vec<Box<dyn FlowNode>>,
    pub data: Option<T>,
}

impl<T> Node<T> {
    pub fn new(name: String) -> Node<T> {
        Node {
            id: Uuid::new_v4().to_string(),
            name: name,
            outputs: vec![],
            data: None,
        }
    }

    pub fn add_output(&mut self, node: Box<dyn FlowNode>) {
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