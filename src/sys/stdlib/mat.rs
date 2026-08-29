use crate::sys::warnings::Error;
use crate::sys::{self, ast};
use talm::unit::Mathrai;

pub fn size_helper(
    n: &ast::ASTNode,
    root: &ast::ASTNode,
    symbols: &sys::SymbolTable
) -> Result<Mathrai, Error> {
    let mut res = Mathrai(0);
    match n {
        ast::ASTNode::Figure(u) => return Ok(Mathrai(*u)),
        ast::ASTNode::FnCall(s) => {
            let pos = symbols.get(s)?;
            let target_fn_node = &root.get_children()[pos];
            res = res + seq_count_m(target_fn_node)?;
        }
        ast::ASTNode::Root(v) => {
            for c in v {
                res = res + size_helper(c, root, symbols)?;
            }
        }
        ast::ASTNode::Gap(v) => {
            for c in v {
                res = res + size_helper(c, root, symbols)?;
            }
        }
        ast::ASTNode::Sequence(_, _) => {
            return Ok(Mathrai(0));
        }
        _ => res = Mathrai(0),
    }
    Ok(res)
}

pub fn seq_count_m(head: &ast::ASTNode) -> Result<Mathrai, Error> {
    let mut res: usize = 0;
    match head {
        ast::ASTNode::Sequence(_, children) => {
            for child in children {
                match child {
                    ast::ASTNode::Figure(u) => res = res + u,
                    ast::ASTNode::Gap(sub_children) => {
                        for sub_child in sub_children {
                            match sub_child {
                                ast::ASTNode::Figure(u) => res = res + u,
                                _ => return Err(Error::global("encountered illegal node in gap")),
                            }
                        }
                    }

                    _ => return Err(Error::global("encountered illegal node in sequence")),
                }
            }
        }
        _ => return Err(Error::global("cannot invoke helper on non-sequence node")),
    }
    Ok(Mathrai(res))
}
