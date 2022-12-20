use std::collections::HashMap;
#[derive(Clone)]
pub enum ObjectValueType {
    Number(i32),
    String(String),
}

pub type MessageType = Option<HashMap<String, ObjectValueType>>;

pub type RunResult = Result<bool, bool>;

pub enum NodeType {
    DebugNode,
    InjectNode,
}

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

    fn print(&self) {
        println!("id: {}", self.get_id());
        println!("name: {}", self.get_name());
    }

    fn print_outputs(&self) {
        println!(
            "id:{}\nname:{}\noutputs:\n[",
            self.get_id(),
            self.get_name()
        );
        for output in self.outputs.as_slice() {
            println!(
                "\t{{\n\t\tid:{}\n\t\tname:{}\n\t}},",
                output.get_id(),
                output.get_name()
            )
        }
        println!("]")
    }
}
