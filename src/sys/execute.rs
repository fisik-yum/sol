use crate::{
    calc,
    sys::{
        ast::{self, NodeType},
        parser::SymbolTable,
        warnings::ParseError,
    },
};

pub fn execute(root: &ast::Node, symbol_table: &SymbolTable) -> Result<(), ParseError> {
    if root.node_type != NodeType::Root {
        panic!("");
    }
    let children = root.children.as_ref().unwrap();
    for child in children {
        match child.node_type {
            NodeType::MatLocal(s) => {
                let pos = symbol_table.get(s)?;
                if let Some(root_children) = &root.children {
                    let target_fn_node = &root_children[pos];
                    // calc::mat::count_m(target_fn_node, root, symbol_table);
                    // exec block
                    let res = calc::mat::seq_count_m(target_fn_node, root, symbol_table)?;
                    println!("Sequence {s}: {}", res)
                } else {
                    panic!("symbol table lookup: undefined behavior - {s}");
                }
            }
            NodeType::MatGlobal => {
                let res = calc::mat::count_m(root, root, symbol_table)?;
                println!("Global: {}", res)
            }
            _ => {}
        }
    }
    Ok(())
}
