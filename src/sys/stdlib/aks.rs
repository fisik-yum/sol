use crate::sys::SymbolTable;
use crate::sys::{ast::ASTNode, stdlib::mat, warnings::Error};
use talm::aks::*;
use talm::unit::Mathrai;

// WARNING: there is something seriously wrong with either this/talm impl
pub fn count_a<'p>(root: &ASTNode, symbols: &SymbolTable) -> Result<StandardAkshara, Error> {
    let mut ret = StandardAkshara {
        count: 0,
        edam: Carry { num: 0, den: 4 },
    };
    let mut accumulator = Mathrai(0);
    let child = root.get_children();

    let mut curr_nad = Mathrai(4);

    for n in child {
        match n {
            ASTNode::Nad(u) => {
                ret = ret + StandardAkshara::from_mathrai(accumulator, curr_nad);
                accumulator = Mathrai(0);
                curr_nad.0 = *u;
            }
            ASTNode::Figure(u) => {
                accumulator = accumulator + Mathrai(*u);
            }
            ASTNode::Gap(_) => {
                let mc = mat::count_m(n, symbols)?;
                accumulator = accumulator + mc;
            }
            ASTNode::FnCall(s) => {
                let target_fn_node = symbols.get(s)?;
                let mc = mat::seq_count_m(target_fn_node)?;
                accumulator = accumulator + mc;
            }
            _ => {}
        }
    }

    ret = ret + StandardAkshara::from_mathrai(accumulator, curr_nad);
    Ok(ret)
}
