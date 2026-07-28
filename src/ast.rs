pub enum NodeType<'nt> {
    Root,
    Sequence(&'nt str),
    FnCall(&'nt str),
    Gap,
    Figure(usize),
}

pub struct Node<'n> {
    node_type: NodeType<'n>,
    pub children: Option<Vec<Node<'n>>>,
}
impl<'n> Node<'n> {
    pub fn new(node_type: NodeType<'n>) -> Self {
        return Self {
            node_type,
            children: Some(vec![]),
        };
    }

    // WARN: Do not call this method as it is possibly broken
    fn sum(&self) -> usize {
        match &self.node_type {
            NodeType::Sequence(_) => return 0,
            NodeType::FnCall(_s) => {
                // make_fn_call(s) to
                // calculate inner sum
                return 0;
            }
            NodeType::Gap => return self.sum_children(),
            NodeType::Root => return self.sum_children(),
            NodeType::Figure(u) => *u,
        }
    }

    fn sum_children(&self) -> usize {
        let mut size: usize = 0;
        if let Some(children) = self.children.as_ref() {
            for node in children {
                size += node.sum()
            }
        }
        return size;
    }
}
