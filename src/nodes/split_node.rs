use std::vec;

use uuid::Uuid;

use crate::nodes::node::{FlowNode, MessageType, Node, RunResult, Runnable};

#[derive(Clone)]
pub enum SplitRuleOperator {
    EQ,
    GT,
    LT,
}

#[derive(Clone)]
pub struct SplitRule {
    key: String,
    operator: SplitRuleOperator,
    value: ObjectValueType,
}

#[derive(Clone)]
pub struct SplitNode {
    rules: Vec<SplitRule>,
}

impl Node<SplitNode> {
    pub fn new(name: String) -> Node<SplitNode> {
        Node {
            id: Uuid::new_v4().to_string(),
            name: name,
            outputs: vec![],
            data: None,
        }
    }
}

impl FlowNode for Node<SplitNode> {
    fn add_output(&mut self, output_node: Box<dyn FlowNode>) {
        self.outputs.push(output_node);
    }
}

impl Runnable for Node<SplitNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        if let Some(some_payload) = payload {
            some_payload.get()
        }
        for split_rule in self.data.as_ref().unwrap().rules.clone() {}
        Ok(true)
    }
}
