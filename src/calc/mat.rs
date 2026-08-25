use crate::sys::ast::Node;
use crate::sys::warnings::ParseError;
use crate::sys::{ast, parser};
use talm::unit::Mathrai;

pub fn count_m(
    head: &ast::Node,
    root: &ast::Node,
    symbol_table: &parser::SymbolTable,
) -> Result<Mathrai, ParseError> {
    size_helper(head, root, symbol_table, false)
}

pub fn size_helper(
    n: &ast::Node,
    root: &ast::Node,
    symbols: &parser::SymbolTable,
    seq_lookup_state: bool,
) -> Result<Mathrai, ParseError> {
    let mut res = Mathrai(0);
    match n.node_type {
        ast::NodeType::Figure(u) => return Ok(Mathrai(u)),
        ast::NodeType::FnCall(s) => {
            let pos = symbols.get(s)?;
            if let Some(root_children) = &root.children {
                let target_fn_node = &root_children[pos];

                res = res + size_helper(target_fn_node, root, symbols, true)?;
            } else {
                panic!("symbol table lookup: undefined behavior - {s}");
            }
        }
        ast::NodeType::Root => {
            for c in n.children.as_ref().unwrap() {
                res = res + size_helper(c, root, symbols, false)?;
            }
        }
        ast::NodeType::Gap => {
            for c in n.children.as_ref().unwrap() {
                res = res + size_helper(c, root, symbols, false)?;
            }
        }
        ast::NodeType::Sequence(_) => {
            if !seq_lookup_state {
                return Ok(Mathrai(0));
            }
            for c in n.children.as_ref().unwrap() {
                res = res + size_helper(c, root, symbols, false)?;
            }
        }
        _ => res = Mathrai(0),
    }
    Ok(res)
}

pub fn seq_count_m(
    head: &ast::Node,
    root: &ast::Node,
    symbols: &parser::SymbolTable,
) -> Result<Mathrai, ParseError> {
    let res = size_helper(head, root, symbols, true)?;
    Ok(res)
}
