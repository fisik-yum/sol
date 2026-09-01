use crate::sys::warnings::Error;
use crate::sys::{Program, ast};
use talm::unit::Mathrai;

pub fn size_helper<'p>(n: &ast::ASTNode<'p>, prog: &Program<'p>) -> Result<Mathrai, Error> {
    let mut res = Mathrai(0);
    match n {
        ast::ASTNode::Figure(u) => return Ok(Mathrai(*u)),
        ast::ASTNode::FnCall(s) => {
            let pos = prog.symbols.get(s)?;
            let target_node = prog.get_root().get_child(pos);
            res = res + seq_count_m(target_node, prog)?;
        }
        ast::ASTNode::Root(v) => {
            for c in v {
                res = res + size_helper(c, prog)?;
            }
        }
        ast::ASTNode::Gap(v) => {
            for c in v {
                res = res + size_helper(c, prog)?;
            }
        }
        ast::ASTNode::Sequence(_, _) => {
            return Ok(Mathrai(0));
        }
        _ => res = Mathrai(0),
    }
    Ok(res)
}

pub fn seq_count_m<'p>(head: &ast::ASTNode<'p>, prog: &Program<'p>) -> Result<Mathrai, Error> {
    let mut res: usize = 0;

    match head {
        ast::ASTNode::Sequence(s, children) => {
            let memo_value = prog.get_memo(s);
            if memo_value.is_some() {
                return Ok(memo_value.unwrap());
            }

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
            prog.set_memo(s, Mathrai(res));
            Ok(Mathrai(res))
        }
        _ => return Err(Error::global("cannot invoke helper on non-sequence node")),
    }
}
