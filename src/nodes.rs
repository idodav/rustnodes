pub enum Nodes {
    Node(Node),
    Inject(Inject),
    Debug(Debug),
    Sql(Sql),
    Http(Http),
}

impl Nodes {
    pub fn to_node(self) -> Node {
        match self {
            Nodes::Node(x) => x,
            _ => panic!("not a Node Node"),
        }
    }
    pub fn to_inject(self) -> Inject {
        match self {
            Nodes::Inject(x) => x,
            _ => panic!("not an Inject Node"),
        }
    }
    pub fn to_debug(self) -> Debug {
        match self {
            Nodes::Debug(x) => x,
            _ => panic!("not a Debug Node"),
        }
    }
    pub fn to_sql(self) -> Sql {
        match self {
            Nodes::Sql(x) => x,
            _ => panic!("not an Sql Node"),
        }
    }
    pub fn to_http(self) -> Http {
        match self {
            Nodes::Http(x) => x,
            _ => panic!("not an Http Node"),
        }
    }

    pub fn run(&self, msg: Option<String>) -> Option<String> {
        match self {
            Nodes::Debug(d) => d.run(msg),
            _ => Some("".to_string()),
        }
    }
}

pub struct Node {
    pub id: u32,
    pub name: String,
    pub out_nodes: Option<Vec<Nodes>>,
}

pub struct Inject {
    pub id: u32,
    pub name: String,
    pub message: String,
    pub out_nodes: Option<Vec<Nodes>>,
}

pub struct Debug {
    pub id: u32,
    pub name: String,
    pub out_nodes: Option<Vec<Nodes>>,
}

pub struct Http {
    pub id: u32,
    pub name: String,
    pub method: String,
    pub path: String,
    pub out_nodes: Option<Vec<Nodes>>,
}

pub struct Sql {
    pub id: u32,
    pub name: String,
    pub query: String,
    pub out_nodes: Option<Vec<Nodes>>,
}

impl Inject {
    pub fn inject(&self) -> Option<String> {
        for node in self.out_nodes.as_ref().unwrap() {
            if let Some(msg) = node.run(Some("this is the first message".to_string())) {
                println!("{}", msg);
            }
        }
        None
    }
}

impl Debug {
    pub fn run(&self, msg: Option<String>) -> Option<String> {
        if let Some(msg) = msg.as_ref() {
            println!("{}", msg);
        }
        None
    }
}
