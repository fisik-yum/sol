use std::{collections::HashMap, rc::Rc};

use crate::sys::{ast::ASTNode, warnings::Error};
use talm::unit::Mathrai;

pub fn count_m<'p>(
    root: &ASTNode<'p>,
    symbols: Rc<HashMap<&'p str, Rc<ASTNode<'p>>>>,
) -> Result<Mathrai, Error<'p>> {
    let mut res = Mathrai(0);
    match root {
        ASTNode::Figure(u) => return Ok(Mathrai(*u)),
        ASTNode::FnCall(s) => {
            let target_node = symbols.get(s).unwrap();
            res = res + seq_count_m(&target_node)?;
        }
        ASTNode::Root(v) => {
            for c in v {
                res = res + count_m(c, symbols.clone())?;
            }
        }
        ASTNode::Sequence(_, _) => {
            return Ok(Mathrai(0));
        }
        _ => res = Mathrai(0),
    }
    Ok(res)
}

pub fn seq_count_m<'p>(head: &ASTNode<'p>) -> Result<Mathrai, Error<'p>> {
    let mut res: usize = 0;

    match head {
        ASTNode::Sequence(_s, children) => {
            for child in children {
                match &**child {
                    ASTNode::Figure(u) => res = res + u,
                    _ => {
                        return Err(Error::global(
                            "encountered illegal node in sequence".to_string(),
                        ));
                    }
                }
            }
            Ok(Mathrai(res))
        }
        _ => {
            return Err(Error::global(
                "cannot invoke helper on non-sequence node".to_string(),
            ));
        }
    }
}
