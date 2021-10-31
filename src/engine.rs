pub mod nodes {
    include!("nodes.rs");
}
pub use nodes::*;

pub struct Flow {
    id: u32,
    name: String,
    nodes: Vec<Nodes>,
}

pub struct Message {
    id: u32,
    payload: String,
}
