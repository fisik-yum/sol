use crate::{
    calc,
    sys::{ast::ASTNode, parser::SymbolTable, warnings::Error},
};

pub fn execute(root: &ASTNode, symbol_table: &SymbolTable) -> Result<(), Error> {
    match root {
        ASTNode::Root(root_children) => {
            //let children = root.children.as_ref().unwrap();
            for child in root_children {
                match child {
                    ASTNode::MatLocal(s) => {
                        let pos = symbol_table.get(s)?;
                        let target_fn_node = &root_children[pos];
                        // calc::mat::count_m(target_fn_node, root, symbol_table);
                        // exec block
                        let res = calc::mat::seq_count_m(target_fn_node, root, symbol_table)?;
                        println!("Sequence {s}: {}", res);
                    }
                    ASTNode::MatGlobal => {
                        let res = calc::mat::count_m(root, root, symbol_table)?;
                        println!("Global: {}", res)
                    }
                    _ => {}
                }
            }
            return Ok(());
        }
        _ => Err(Error::global("cannot execute")),
    }
}
