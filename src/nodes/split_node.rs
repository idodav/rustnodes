use std::vec;

use uuid::Uuid;

use crate::nodes::node::{FlowNode, MessageType, Node, RunResult, Runnable};

use super::node::ObjectValueType;

#[derive(Clone, Debug)]
pub enum SplitRuleOperator {
    EQ,
    GT,
    LT,
}

#[derive(Clone, Debug)]
pub struct SplitRule {
    pub key: String,
    pub operator: SplitRuleOperator,
    pub value: ObjectValueType,
}

#[derive(Clone)]
pub struct SplitNode {
    pub rules: Vec<SplitRule>,
}

impl Node<SplitNode> {
    pub fn new(name: String, data: Option<SplitNode>) -> Node<SplitNode> {
        Node {
            id: Uuid::new_v4().to_string(),
            name,
            outputs: vec![],
            data,
        }
    }
}

impl FlowNode for Node<SplitNode> {
    fn add_output(&mut self, output_node: Box<dyn FlowNode>) {
        self.outputs.push(output_node);
    }
}

fn EQRule(value1: Option<&ObjectValueType>, value2: &ObjectValueType) -> bool {
    return value2.eq(value1.unwrap());
}
fn GTRule(value1: Option<&ObjectValueType>, value2: &ObjectValueType) -> bool {
    return value2.gt(value1.unwrap());
}
fn LTRule(value1: Option<&ObjectValueType>, value2: &ObjectValueType) -> bool {
    return value2.lt(value1.unwrap());
}

impl Runnable for Node<SplitNode> {
    fn run(&self, payload: MessageType) -> RunResult {
        println!("running split node");

        if let Some(some_payload) = &payload {
            for (i, split_rule) in self.data.as_ref().unwrap().rules.iter().enumerate() {
                println!(
                    "checking rule:{} {:?} {:?} {:?}",
                    i, split_rule.key, split_rule.operator, split_rule.value
                );

                let ruleResult: bool = match split_rule.operator {
                    SplitRuleOperator::EQ => {
                        EQRule(some_payload.get(&split_rule.key), &split_rule.value)
                    }

                    SplitRuleOperator::GT => {
                        GTRule(some_payload.get(&split_rule.key), &split_rule.value)
                    }
                    SplitRuleOperator::LT => {
                        LTRule(some_payload.get(&split_rule.key), &split_rule.value)
                    }
                };

                if ruleResult == true {
                    println!(
                        "rule check passed:{:?} {:?} {:?} running output[{}] node",
                        split_rule.key, split_rule.operator, split_rule.value, i
                    );
                    if let Some(output_node) = self.outputs.get(i) {
                        output_node.run(payload);
                    } else {
                        println!("output[{}] is not defined", i);
                    }
                    break;
                }
            }
        }

        Ok(true)
    }
}
