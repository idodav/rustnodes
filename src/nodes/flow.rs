use crate::nodes::node::FlowNode;

pub struct Flow<'a> {
    pub nodes: Vec<&'a Box<dyn FlowNode>>,
}