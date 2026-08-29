use crate::sys::{self, ast::{self, ASTNode}, stdlib::mat, warnings::Error};
use talm::aks::*;
use talm::unit::Mathrai;

pub fn count_a(
    root: &ast::ASTNode,
    symbol_table: &sys::SymbolTable,
) -> Result<StandardAkshara, Error> {
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
                let mc = mat::size_helper(n, root, symbol_table)?;
                accumulator = accumulator + mc;
            }
            ASTNode::FnCall(s) => {
                let pos = symbol_table.get(s)?;
                let target_fn_node = &child[pos];
                let mc = mat::seq_count_m(target_fn_node)?;
                accumulator = accumulator + mc;
            }
            _ => {}
        }
    }

    ret = ret + StandardAkshara::from_mathrai(accumulator, curr_nad);
    Ok(ret)
}
