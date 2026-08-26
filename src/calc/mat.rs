use crate::sys::warnings::Error;
use crate::sys::{ast, parser};
use talm::unit::Mathrai;

pub fn count_m(
    head: &ast::ASTNode,
    root: &ast::ASTNode,
    symbol_table: &parser::SymbolTable,
) -> Result<Mathrai, Error> {
    size_helper(head, root, symbol_table, false)
}

pub fn size_helper(
    n: &ast::ASTNode,
    root: &ast::ASTNode,
    symbols: &parser::SymbolTable,
    seq_lookup_state: bool,
) -> Result<Mathrai, Error> {
    let mut res = Mathrai(0);
    match n {
        ast::ASTNode::Figure(u) => return Ok(Mathrai(*u)),
        ast::ASTNode::FnCall(s) => {
            let pos = symbols.get(s)?;
            let target_fn_node = &root.get_children()[pos];
            res = res + size_helper(target_fn_node, root, symbols, true)?;
        }
        ast::ASTNode::Root(v) => {
            for c in v {
                res = res + size_helper(c, root, symbols, false)?;
            }
        }
        ast::ASTNode::Gap(v) => {
            for c in v {
                res = res + size_helper(c, root, symbols, false)?;
            }
        }
        ast::ASTNode::Sequence(_s,v) => {
            if !seq_lookup_state {
                return Ok(Mathrai(0));
            }
            for c in v {
                res = res + size_helper(c, root, symbols, false)?;
            }
        }
        _ => res = Mathrai(0),
    }
    Ok(res)
}

pub fn seq_count_m(
    head: &ast::ASTNode,
    root: &ast::ASTNode,
    symbols: &parser::SymbolTable,
) -> Result<Mathrai, Error> {
    let res = size_helper(head, root, symbols, true)?;
    Ok(res)
}
