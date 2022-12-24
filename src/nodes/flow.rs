use crate::nodes::node::FlowNode;

pub struct Flow<'a> {
    pub id: String,
    pub name: String,
    pub nodes: Vec<&'a Box<dyn FlowNode>>,
}

impl<'a> Flow<'a> {
    pub fn set_nodes(&mut self, nodes: Vec<&'a Box<dyn FlowNode>>) {
        self.nodes = nodes;
    }
    pub fn get_node_by_id(&self, id: String) -> Option<&&Box<dyn FlowNode>> {
        self.nodes.iter().find(|node| node.get_id().eq(&id))
    }
}
