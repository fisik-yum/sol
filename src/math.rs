use crate::{ast, parse};

pub fn count_m(head: &ast::Node, root: &ast::Node, symbol_table: &parse::SymbolTable) -> usize {
    return size_helper(head, root, symbol_table, false);
}

fn size_helper(
    n: &ast::Node,
    root: &ast::Node,
    symbols: &parse::SymbolTable,
    seq_lookup_state: bool,
) -> usize {
    let mut res: usize = 0;
    match n.node_type {
        ast::NodeType::Figure(u) => return u,
        ast::NodeType::FnCall(s) => {
            let pos = symbols.get(s);
            if let Some(root_children) = &root.children {
                let target_fn_node = &root_children[pos];

                res += size_helper(target_fn_node, root, symbols, true);
            } else {
                panic!("symbol table lookup: undefined behavior - {s}");
            }
        }
        ast::NodeType::Root => {
            for c in n.children.as_ref().unwrap() {
                res = res + size_helper(c, root, symbols, false);
            }
        }
        ast::NodeType::Gap => {
            for c in n.children.as_ref().unwrap() {
                res = res + size_helper(c, root, symbols, false);
            }
        }
        ast::NodeType::Sequence(_) => {
            if !seq_lookup_state {
                return 0;
            }
            for c in n.children.as_ref().unwrap() {
                res = res + size_helper(c, root, symbols, false);
            }
        }
    }
    res
}
