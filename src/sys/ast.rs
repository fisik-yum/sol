#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode<'t> {
    Root(Vec<Self>),
    Tal(usize),
    Nad(usize),
    MatLocal(&'t str),
    MatGlobal,
    Aks,
    Sequence(&'t str, Vec<Self>),
    FnCall(&'t str),
    Gap(Vec<Self>),
    Figure(usize),
}

impl<'t> std::fmt::Display for ASTNode<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Root(_) => write!(f, ""),
            ASTNode::Sequence(s, _) => write!(f, "seq {s}"),
            ASTNode::Tal(u) => write!(f, "tal {u}"),
            ASTNode::Nad(u) => write!(f, "nad {u}"),
            ASTNode::MatLocal(u) => write!(f, "mat {u}"),
            ASTNode::MatGlobal => write!(f, "mat sol"),
            ASTNode::Aks => write!(f, "aks"),
            ASTNode::FnCall(s) => write!(f, "{s}"),
            ASTNode::Gap(_) => write!(f, "gap"),
            ASTNode::Figure(n) => write!(f, "{n}"),
        }
    }
}

impl<'n> ASTNode<'n> {
    pub fn insert_node(&mut self, n: ASTNode<'n>) -> usize {
        match self {
            ASTNode::Root(v) => {
                v.push(n);
                v.len() - 1
            }
            ASTNode::Sequence(_, v) => {
                v.push(n);
                v.len() - 1
            }
            ASTNode::Gap(v) => {
                v.push(n);
                v.len() - 1
            }
            _ => {
                panic!("unexpected behavior")
            }
        }
    }

    pub fn get_children(&self) -> &Vec<Self> {
        match self {
            ASTNode::Sequence(_, v) => v,
            ASTNode::Root(v) => v,
            ASTNode::Gap(v) => v,
            _ => panic!("get_children called on a non-Sequence node: {}", self),
        }
    }
    pub fn get_name(&self) -> &'n str {
        match self {
            ASTNode::Sequence(s, _) => s,
            _ => panic!("get_name called on a non-Sequence node: {}", self),
        }
    }
    // pub fn prettyprint(&self) {
    //     self.dfs("", true);
    // }

    // fn dfs(&self, prefix: &str, is_last: bool) {
    //     let connector = if is_last { "\\_" } else { "|- " };

    //     println!("{prefix}{connector}{}", self.node_type);

    //     let child_prefix = format!("{prefix}{}", if is_last { "    " } else { "|    " });

    //     if let Some(children) = &self.children {
    //         let count = children.len();
    //         for (i, child) in children.iter().enumerate() {
    //             let is_last_child = i == count - 1;
    //             child.dfs(&child_prefix, is_last_child);
    //         }
    //     }
    // }
}
