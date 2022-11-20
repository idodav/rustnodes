use std::collections::HashMap;
mod nodes;
use nodes::flow::Flow;
use nodes::debug_node::DebugNode;
use nodes::node::{FlowNode,Node, Runnable,BasicNode};
use nodes::inject_node::InjectNode;

fn main() {
    let mut flow = Flow { nodes: vec![] };

    let mut debug_node: Node<DebugNode> = Node::new("debug".to_string());
    let mut inject_node: Node<InjectNode> = Node::new("inject".to_string());
    let debug_flow_node: Box<FlowNode> = Box::new(debug_node.clone());
    let inject_flow_node: Box<FlowNode> = Box::new(inject_node.clone());

    flow.nodes.push(&debug_flow_node);
    flow.nodes.push(&inject_flow_node);

    debug_node.data = Some(DebugNode {
        message: "hello".to_string(),
        output_keys: vec!["key1".to_string(), "key2".to_string()],
    });

    let mut inject_data = HashMap::new();

    inject_data.insert("key1".to_string(), "some data".to_string());
    inject_data.insert("key2".to_string(), "some data 2".to_string());

    inject_node.data = Some(InjectNode {
        payload: Some(inject_data),
    });

    inject_node.add_output(Box::new(debug_node));
    inject_node.run(None);
}
