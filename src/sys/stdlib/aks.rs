use crate::sys::{self, ast::{ASTNode}, stdlib::mat, warnings::Error};
use talm::aks::*;
use talm::unit::Mathrai;

pub fn count_a<'p>(
    prog: &sys::Program<'p>,
) -> Result<StandardAkshara, Error> {
    let mut ret = StandardAkshara {
        count: 0,
        edam: Carry { num: 0, den: 4 },
    };
    let mut accumulator = Mathrai(0);
    let child = prog.root.get_children();

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
                let mc = mat::size_helper(n, prog)?;
                accumulator = accumulator + mc;
            }
            ASTNode::FnCall(s) => {
                let pos = prog.symbols.get(s)?;
                let target_fn_node = &child[pos];
                let mc = mat::seq_count_m(target_fn_node, prog)?;
                accumulator = accumulator + mc;
            }
            _ => {}
        }
    }

    ret = ret + StandardAkshara::from_mathrai(accumulator, curr_nad);
    Ok(ret)
}
