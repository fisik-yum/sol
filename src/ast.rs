#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType<'t> {
    Root,
    Tal(usize),
    Nad(usize),
    Sequence(&'t str),
    FnCall(&'t str),
    Gap,
    Figure(usize),
}

impl<'t> std::fmt::Display for NodeType<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Root => write!(f, "Root"),
            NodeType::Sequence(s) => write!(f, "Sequence({s})"),
            NodeType::Tal(u) => write!(f, "Tal({u}a)"),
            NodeType::Nad(u) => write!(f, "Nad({u}m)"),
            NodeType::FnCall(s) => write!(f, "FnCall({s})"),
            NodeType::Gap => write!(f, "Gap"),
            NodeType::Figure(n) => write!(f, "Figure({n})"),
        }
    }
}

pub struct Node<'n> {
    pub node_type: NodeType<'n>,
    pub children: Option<Vec<Node<'n>>>,
}

impl<'n> Node<'n> {
    pub fn new(node_type: NodeType<'n>) -> Self {
        return Self {
            node_type,
            children: Some(vec![]),
        };
    }

    pub fn insert_node(&mut self, n: Node<'n>) -> usize {
        let ch = self.children.get_or_insert_with(Vec::new);
        ch.push(n);
        ch.len() - 1
    }

    pub fn get_name(&self) -> &'n str {
        match self.node_type {
            NodeType::Sequence(s) => s,
            _ => panic!(""),
        }
    }
    pub fn prettyprint(&self) {
        self.dfs("", true);
    }

    fn dfs(&self, prefix: &str, is_last: bool) {
        let connector = if is_last { "\\_" } else { "|- " };

        println!("{prefix}{connector}{}", self.node_type);

        let child_prefix = format!("{prefix}{}", if is_last { "    " } else { "|    " });

        if let Some(children) = &self.children {
            let count = children.len();
            for (i, child) in children.iter().enumerate() {
                let is_last_child = i == count - 1;
                child.dfs(&child_prefix, is_last_child);
            }
        }
    }
}
