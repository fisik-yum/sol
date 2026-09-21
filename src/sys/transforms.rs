use crate::sys::{ast::ASTNode, warnings::Error};

pub trait Transform {
    fn mutate<'m>(&self, _root: ASTNode<'m>) -> Result<ASTNode<'m>, Error> {
        Err(Error::global("invalid transform"))
    }
}

pub struct RemoveInteractive;

impl RemoveInteractive {
    pub fn new() -> Self {
        Self
    }
}
impl Transform for RemoveInteractive {
    fn mutate<'m>(&self, mut root: ASTNode<'m>) -> Result<ASTNode<'m>, Error> {
        let filter: Vec<ASTNode> = root
            .get_children()
            .iter()
            .cloned()
            .filter(|n| !n.is_interactive())
            .collect();
        root.set_children(filter);
        Ok(root)
    }
}

pub struct KeepInteractive;
impl KeepInteractive {
    pub fn new() -> Self {
        Self
    }
}
impl Transform for KeepInteractive {
    fn mutate<'m>(&self, mut root: ASTNode<'m>) -> Result<ASTNode<'m>, Error> {
        let filter: Vec<ASTNode> = root
            .get_children()
            .iter()
            .cloned()
            .filter(|n| !n.is_interactive())
            .collect();
        root.set_children(filter);
        Ok(root)
    }
}

pub struct InteractiveMode;
impl InteractiveMode {
    pub fn new() -> Self {
        Self
    }
}
impl Transform for InteractiveMode {
    fn mutate<'m>(&self, mut root: ASTNode<'m>) -> Result<ASTNode<'m>, Error> {
        let filter: Vec<ASTNode> = root
            .get_children()
            .iter()
            .cloned()
            .filter(|n| matches!(n, ASTNode::Sequence(_, _)) || n.is_interactive())
            .collect();
        root.set_children(filter);
        Ok(root)
    }
}
