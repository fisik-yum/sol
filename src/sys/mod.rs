pub mod ast;
pub mod parser;
pub mod stdlib;
pub mod tokenize;
pub mod warnings;

use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use talm::unit::Mathrai;

use crate::sys::warnings::Error;

pub struct Program<'p> {
    pub root: ast::ASTNode<'p>,
    pub symbols: SymbolTable<'p>,
    pub memo: RefCell<HashMap<&'p str, Mathrai>>,
}
impl<'p> Program<'p> {
    pub fn new(
        root: ast::ASTNode<'p>,
        symbols: SymbolTable<'p>,
        memo: HashMap<&'p str, Mathrai>,
    ) -> Self {
        Self {
            root,
            symbols,
            memo: RefCell::new(memo),
        }
    }
    pub fn get_root(&self) -> &ast::ASTNode<'p> {
        return &self.root;
    }

    pub fn get_memo(&self, key: &str) -> Option<Mathrai> {
        // should convert to result and warning?
        self.memo.borrow().get(key).cloned()
    }

    pub fn set_memo(&self, key: &'p str, value: Mathrai) {
        self.memo.borrow_mut().insert(key, value);
    }

    pub fn mathrai_count(&self) -> Result<Mathrai, Error> {
        stdlib::mat::size_helper(&self.root, self)
    }
    pub fn akshara_count() {}
}
pub struct SymbolTable<'a> {
    table: HashMap<&'a str, usize>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    pub fn insert(&mut self, k: &'a str, idx: usize, pos: usize) -> Result<(), Error> {
        if self.table.contains_key(k) {
            return Err(Error::at(pos, format!("sequence redefined: {k}")));
        }
        self.table.insert(k, idx);
        Ok(())
    }

    pub fn get(&self, k: &'a str) -> Result<usize, Error> {
        self.table
            .get(k)
            .copied()
            .ok_or_else(|| Error::global(format!("undefined sequence: {k}")))
    }
}

impl std::fmt::Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.table.fmt(f)
    }
}
