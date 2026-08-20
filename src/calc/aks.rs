use crate::{
    ast::{self, NodeType},
    calc::mat::{count_m, size_helper},
    parser,
};
use talm::unit::{Aks, Mat};

pub fn count_a(root: &ast::Node, symbol_table: &parser::SymbolTable) -> Aks {
    let mut ret = Aks::new(0, Mat(0));
    let chld = root.children.as_ref().unwrap();

    let mut c_nadai = Mat(4);

    for n in chld {
        match n.node_type {
            NodeType::Nad(u) => {
                c_nadai.0 = u;
            }
            NodeType::Figure(u) => {
                ret = ret + Aks::from_n_m(c_nadai, Mat(u));
            }
            NodeType::Gap => {
                let mc = count_m(n, root, symbol_table);
                ret = ret + Aks::from_n_m(c_nadai, mc);
            }
            NodeType::FnCall(s) => {
                let pos = symbol_table.get(s);
                if let Some(root_children) = &root.children {
                    let target_fn_node = &root_children[pos];
                    let mc = size_helper(target_fn_node, root, symbol_table, true);
                    ret = ret + Aks::from_n_m(c_nadai, mc);
                } else {
                    panic!("symbol table lookup: undefined behavior - {s}");
                }
            }
            _ => {}
        }
    }
    return ret;
}
