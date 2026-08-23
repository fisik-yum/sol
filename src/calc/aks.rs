use crate::{
    sys::ast::{self, NodeType},
    calc::mat::{count_m, size_helper},
    sys::parser,
    sys::warnings::ParseError,
};
use talm::aks::*;
use talm::unit::Mathrai;

pub fn count_a(
    root: &ast::Node,
    symbol_table: &parser::SymbolTable,
) -> Result<StandardAkshara, ParseError> {
    let mut ret = StandardAkshara {
        count: 0,
        edam: Carry { num: 0, den: 4 },
    };
    let mut accumulator = Mathrai(0);
    let chld = root.children.as_ref().unwrap();

    let mut curr_nad = Mathrai(4);

    for n in chld {
        match n.node_type {
            NodeType::Nad(u) => {
                ret = ret + StandardAkshara::from_mathrai(accumulator, curr_nad);
                accumulator = Mathrai(0);
                curr_nad.0 = u;
            }
            NodeType::Figure(u) => {
                accumulator = accumulator + Mathrai(u);
            }
            NodeType::Gap => {
                let mc = count_m(n, root, symbol_table)?;
                accumulator = accumulator + mc;
            }
            NodeType::FnCall(s) => {
                let pos = symbol_table.get(s)?;
                if let Some(root_children) = &root.children {
                    let target_fn_node = &root_children[pos];
                    let mc = size_helper(target_fn_node, root, symbol_table, true)?;
                    accumulator = accumulator + mc;
                } else {
                    panic!("symbol table lookup: undefined behavior - {s}");
                }
            }
            _ => {}
        }
    }

    ret = ret + StandardAkshara::from_mathrai(accumulator, curr_nad);
    Ok(ret)
}
