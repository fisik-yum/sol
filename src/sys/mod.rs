pub mod ast;
pub mod parser;
pub mod stdlib;
pub mod tokenize;
pub mod transforms;
pub mod warnings;

use std::{collections::HashMap, fmt::Debug};

use crate::sys::{ast::ASTNode, warnings::Error};

pub struct Pipeline<'p> {
    transform_path: Vec<&'p dyn transforms::Transform>,
}

impl<'p> Pipeline<'p> {
    pub fn new() -> Self {
        Self {
            transform_path: vec![],
        }
    }

    pub fn add_stage(&mut self, stage: &'p dyn transforms::Transform) {
        self.transform_path.push(stage);
    }

    pub fn ingest(&self, mut input: ASTNode<'p>) -> Result<ASTNode<'p>, Error> {
        for stage in self.transform_path.as_slice() {
            input = stage.mutate(input).unwrap();
        }
        Ok(input)
    }
}

pub struct Environment<'p> {
    pipe: Pipeline<'p>,
}
impl<'p> Environment<'p> {
    pub fn new(p: Pipeline<'p>) -> Self {
        Self { pipe: p }
    }
    pub fn interpret(root: ASTNode<'p>) {}
}

pub struct SymbolTable<'a> {
    table: HashMap<&'a str, &'a ASTNode<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new(root: &'a ASTNode<'a>) -> Result<Self,Error> {
        let mut ret = Self {
            table: HashMap::new(),
        };

        let children = root.get_children().as_slice();

        for i in 0..children.len() {
            let child = &children[i];
            match child {
                ASTNode::Sequence(s, _) => {
                    ret.insert(s, child)?;
                }
                _ => {}
            };
        }
        Ok(ret)
    }
    fn insert(&mut self, k: &'a str, idx: &'a ASTNode<'a>) -> Result<(), Error> {
        if self.table.contains_key(k) {
            return Err(Error::global(format!("sequence redefined: {k}")));
        }
        self.table.insert(k, idx);
        Ok(())
    }

    pub fn get(&self, k: &'a str) -> Result<&'a ASTNode<'a>, Error> {
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
