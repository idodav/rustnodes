mod nodes;
use std::collections::HashMap;

use nodes::debug_node::DebugNode;
use nodes::flow::Flow;
use nodes::inject_node::InjectNode;
use nodes::node::{FlowNode, Node, ObjectValueType};
use nodes::split_node::{SplitNode, SplitRule};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

pub struct ServerState<'a> {
    flow: Flow<'a>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let server_state = ServerState {
        flow: Flow {
            id: "123".to_string(),
            name: "flow1".to_string(),
            nodes: vec![],
        },
    };

    rocket::build().mount("/", routes![index])
}

fn create_and_run_flow() {
    let mut flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "flow1".to_string(),
        nodes: vec![],
    };

    let mut injectPayload = HashMap::new();
    injectPayload.insert(
        "SplitTo1".to_string(),
        ObjectValueType::String("true".to_string()),
    );
    injectPayload.insert("SplitTo2".to_string(), ObjectValueType::Number(1));

    let mut split_node_data = SplitNode {
        rules: vec![
            SplitRule {
                key: "SplitTo1".to_string(),
                operator: nodes::split_node::SplitRuleOperator::EQ,
                value: ObjectValueType::String("false".to_string()),
            },
            SplitRule {
                key: "SplitTo2".to_string(),
                operator: nodes::split_node::SplitRuleOperator::GT,
                value: ObjectValueType::Number(1),
            },
            SplitRule {
                key: "SplitTo1".to_string(),
                operator: nodes::split_node::SplitRuleOperator::EQ,
                value: ObjectValueType::String("false".to_string()),
            },
        ],
    };
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

    let mut split_node: Box<dyn FlowNode> = Box::new(Node::<SplitNode>::new(
        "split node".to_string(),
        Some(split_node_data),
    ));

    split_node.add_output(debug_node1);
    split_node.add_output(debug_node2);
    split_node.add_output(debug_node3);
    inject_node.add_output(split_node);
    flow.set_nodes(vec![&inject_node]);

    let a = flow.get_node_by_id(inject_node.get_id());
    a.unwrap().run(None);
}
