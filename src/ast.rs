pub enum NodeType<'t> {
    Root,
    Sequence(&'t str),
    FnCall(&'t str),
    Gap,
    Figure(usize),
}

pub struct Node<'n> {
    node_type: NodeType<'n>,
    pub children: Option<Vec<Node<'n>>>,
    //symbol_table: &'n hash_map::HashMap<&'n str, Node<'n>>,
}
impl<'n> Node<'n> {
    pub fn new(node_type: NodeType<'n>) -> Self {
        return Self {
            node_type,
            children: Some(vec![]),
        };
    }

    pub fn insert_node(&mut self, n: Node<'n>) {
        if let Some(ch) = &mut self.children {
            ch.push(n);
        }
    }
}
