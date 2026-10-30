use crate::sys::ast::ASTNode;
use crate::sys::warnings::Error;
use crate::sys::{stdlib, transforms};
use std::collections::HashMap;
use talm::aks::{Carry, StandardAkshara};
use talm::unit::Mathrai;

pub enum Op {
    TAL(usize),
    NAD(Mathrai),
    FN(String),
    BLOCK(Mathrai),
    RELOAD,
    LOAD(String),
    INSPECT(String),
}

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

    pub fn ingest(&self, mut input: ASTNode<'p>) -> Result<ASTNode<'p>, Error<'p>> {
        for stage in self.transform_path.as_slice() {
            input = stage.mutate(input).unwrap();
        }
        Ok(input)
    }
    pub fn to_bytecode(
        &self,
        root: ASTNode<'p>,
    ) -> Result<(Vec<Op>, HashMap<&'p str, Mathrai>), Error<'p>> {
        let mut ops: Vec<Op> = vec![];
        let mut children = root.get_children().iter().peekable();
        let mut memo: HashMap<&'p str, Mathrai> = HashMap::new();
        while let Some(node) = children.peek() {
            match node {
                ASTNode::Tal(u) => {
                    ops.push(Op::TAL(*u));
                }
                ASTNode::Nad(u) => {
                    ops.push(Op::NAD(Mathrai(*u)));
                }
                ASTNode::Reload => {
                    ops.push(Op::RELOAD);
                }
                ASTNode::Load(s) => {
                    ops.push(Op::LOAD(s.to_string()));
                }
                ASTNode::Inspect(s) => {
                    ops.push(Op::INSPECT(s.to_string()));
                }
                ASTNode::Sequence(s, _) => {
                    let c = stdlib::mat::seq_count_m(node)?;
                    memo.insert(s, c);
                }
                ASTNode::Figure(u) => {
                    // stop-gap. ideally consolidate into one block
                    ops.push(Op::BLOCK(Mathrai(*u)));
                }
                ASTNode::FnCall(s) => {
                    ops.push(Op::FN(s.to_string()));
                }
                _ => {}
            }
            children.next();
        }
        Ok((ops, memo))
    }
}

pub struct Environment<'src> {
    memory: HashMap<&'src str, Mathrai>,
    nadai: Mathrai,
    talam: Option<usize>,
}
impl<'src> Environment<'src> {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            nadai: Mathrai(4),
            talam: None,
        }
    }
    pub fn ingest_memory(&mut self, mem: HashMap<&'src str, Mathrai>) {
        self.memory.extend(mem);
    }
    pub fn execute(&mut self, ops: Vec<Op>) -> Result<(Mathrai, StandardAkshara), Error<'src>> {
        let mut tape = ops.iter();
        let mut aks = StandardAkshara {
            count: 0,
            edam: Carry { num: 0, den: 4 },
        };
        let mut mat = Mathrai(0);
        while let Some(op) = tape.next() {
            match op {
                Op::TAL(u) => {
                    if self.talam.is_some() {
                        return Err(Error::environment("cannot redefine tal".to_string()));
                    }
                    self.talam = Some(*u)
                }
                Op::NAD(u) => {
                    self.nadai = *u;
                }
                Op::BLOCK(u) => {
                    mat = mat + *u;
                    aks = aks + StandardAkshara::from_mathrai(*u, self.nadai)
                }
                Op::FN(s) => {
                    if let Some(u) = self.memory.get(s.as_str()) {
                        mat = mat + *u;
                        aks = aks + StandardAkshara::from_mathrai(*u, self.nadai)
                    }
                }
                _ => {
                    // TODO
                    }
            }
        }
        return Ok((mat, aks));
    }
}
