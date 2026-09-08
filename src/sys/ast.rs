#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode<'t> {
    Root(Vec<Self>),
    Tal(usize),
    Nad(usize),
    Sequence(&'t str, Vec<Self>),
    FnCall(&'t str),
    Gap(Vec<Self>),
    Figure(usize),

    Reload,
    Load(&'t str),
    Inspect(&'t str),
}

impl<'t> std::fmt::Display for ASTNode<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Root(_) => write!(f, ""),
            ASTNode::Sequence(s, _) => write!(f, "seq {s}"),
            ASTNode::Tal(u) => write!(f, "tal {u}"),
            ASTNode::Nad(u) => write!(f, "nad {u}"),
            ASTNode::FnCall(s) => write!(f, "{s}"),
            ASTNode::Gap(_) => write!(f, "gap"),
            ASTNode::Figure(n) => write!(f, "{n}"),

            ASTNode::Reload => write!(f, "reload"),
            ASTNode::Load(s) => write!(f, "load {s}"),
            ASTNode::Inspect(s) => write!(f, "inspect {s}"),
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
    pub fn set_children(&mut self, nc: Vec<Self>) {
        match self {
            ASTNode::Sequence(_, v) => *v = nc,
            ASTNode::Root(v) => *v = nc,
            ASTNode::Gap(v) => *v = nc,
            _ => panic!("set_children called on a non-Sequence node: {}", self),
        }
    }
    pub fn get_child(&self, idx: usize) -> &Self {
        let c = match self {
            ASTNode::Sequence(_, v) => v,
            ASTNode::Root(v) => v,
            ASTNode::Gap(v) => v,
            _ => panic!("get_children called on a non-Sequence node: {}", self),
        };
        return &c[idx];
    }
    pub fn get_name(&self) -> &'n str {
        match self {
            ASTNode::Sequence(s, _) => s,
            _ => panic!("get_name called on a non-Sequence node: {}", self),
        }
    }

    pub fn is_interactive(&self) -> bool {
        matches!(self, Self::Reload | Self::Load(_) | Self::Inspect(_))
    }

    pub fn prettyprint(&self) {
        self.dfs("", true);
    }

    fn dfs(&self, prefix: &str, is_last: bool) {
        let connector = if is_last { "\\_" } else { "|- " };

        println!("{prefix}{connector}{}", self);

        let child_prefix = format!("{prefix}{}", if is_last { "    " } else { "|    " });

        match self {
            ASTNode::Root(v) => {
                let count = v.len();
                for (i, child) in v.iter().enumerate() {
                    let is_last_child = i == count - 1;
                    child.dfs(&child_prefix, is_last_child);
                }
            }
            ASTNode::Sequence(_, v) => {
                let count = v.len();
                for (i, child) in v.iter().enumerate() {
                    let is_last_child = i == count - 1;
                    child.dfs(&child_prefix, is_last_child);
                }
            }
            ASTNode::Gap(v) => {
                let count = v.len();
                for (i, child) in v.iter().enumerate() {
                    let is_last_child = i == count - 1;
                    child.dfs(&child_prefix, is_last_child);
                }
            }
            _ => {}
        }
    }
}
