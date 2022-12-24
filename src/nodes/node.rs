use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ObjectValueType {
    Number(i32),
    String(String),
}

pub type MessageType = Option<HashMap<String, ObjectValueType>>;

pub type RunResult = Result<bool, bool>;

pub trait BasicNode {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
}

pub trait Runnable {
    fn run(&self, payload: MessageType) -> RunResult;
}

pub trait FlowNode: BasicNode + Runnable {
    fn add_output(&mut self, output_node: Box<dyn FlowNode>);
}

pub struct Node<T> {
    pub id: String,
    pub name: String,
    pub outputs: Vec<Box<dyn FlowNode>>,
    pub data: Option<T>,
}

impl<'a, T> BasicNode for Node<T> {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
