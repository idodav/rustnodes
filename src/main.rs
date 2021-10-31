mod engine;
use engine::nodes::*;

fn main() {
    let c = Nodes::Debug(Debug {
        id: 1234,
        name: "second node".to_string(),
        out_nodes: None,
    });

    let b = Nodes::Debug(Debug {
        id: 1234,
        name: "second node".to_string(),
        out_nodes: None,
    });

    let a = Nodes::Inject(Inject {
        id: 1234,
        name: "first node".to_string(),
        message: "injected first message".to_string(),
        out_nodes: Some(vec![b, c]),
    });

    a.to_inject().inject();
}
