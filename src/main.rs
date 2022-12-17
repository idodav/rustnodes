mod nodes;
use std::collections::HashMap;

use nodes::debug_node::DebugNode;
use nodes::flow::Flow;
use nodes::inject_node::InjectNode;
use nodes::node::{FlowNode, Node};
use nodes::split_node::SplitNode;

fn main() {
    let mut flow = Flow { nodes: vec![] };
    let mut injectPayload = HashMap::new();
    injectPayload.insert("hello".to_string(), "you".to_string());
    let mut inject_node: Box<dyn FlowNode> = Box::new(Node::<InjectNode>::new(
        "hello".to_string(),
        Some(InjectNode {
            payload: Some(injectPayload),
        }),
    ));
    let debug_node: Box<dyn FlowNode> = Box::new(Node::<DebugNode>::new("debug".to_string(), None));
    let debug_node1: Box<dyn FlowNode> = Box::new(Node::<DebugNode>::new(
        "debug1".to_string(),
        Some(DebugNode {
            message: "Hello this is debug 1".to_string(),
            output_keys: vec![],
        }),
    ));
    let debug_node2: Box<dyn FlowNode> = Box::new(Node::<DebugNode>::new(
        "debug2".to_string(),
        Some(DebugNode {
            message: "Hello this is debug 1".to_string(),
            output_keys: vec![],
        }),
    ));
    let debug_node3: Box<dyn FlowNode> = Box::new(Node::<DebugNode>::new(
        "debug3".to_string(),
        Some(DebugNode {
            message: "Hello this is debug 1".to_string(),
            output_keys: vec!["hello".to_string()],
        }),
    ));

    let split_node: Box<dyn FlowNode> = Box::new(Node::<SplitNode>::new("split node".to_string()));

    inject_node.add_output(debug_node);
    inject_node.add_output(debug_node1);
    inject_node.add_output(debug_node2);
    inject_node.add_output(debug_node3);

    flow.set_nodes(vec![&inject_node]);

    let a = flow.get_node_by_id(inject_node.get_id());
    a.unwrap().run(None);
}
