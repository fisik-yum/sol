use crate::sys::ast::ASTNode;
use crate::sys::warnings::Error;
use crate::sys::{stdlib, transforms};
use std::collections::HashMap;
use std::fmt::Display;
use talm::aks::{Carry, StandardAkshara};
use talm::unit::Mathrai;

pub enum Op {
    TAL(usize),
    NAD(Mathrai),
    FN(String),
    BLOCK(Mathrai),
    INSPECT(String),
    RESET,
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
        while let Some(node) = children.next() {
            match node {
                ASTNode::Tal(u) => {
                    ops.push(Op::TAL(*u));
                }
                ASTNode::Nad(u) => {
                    ops.push(Op::NAD(Mathrai(*u)));
                }
                ASTNode::Inspect(s) => {
                    ops.push(Op::INSPECT(s.to_string()));
                }
                ASTNode::Sequence(s, _) => {
                    let c = stdlib::mat::seq_count_m(node)?;
                    memo.insert(s, c);
                }
                ASTNode::Figure(u) => {
                    // consolidate into one block
                    let mut c = Mathrai(*u);
                    while let Some(node) = children.peek() {
                        match node {
                            ASTNode::Figure(u) => {
                                c = c + Mathrai(*u);
                                children.next();
                            }
                            _ => break,
                        }
                    }
                    ops.push(Op::BLOCK(c));
                }
                ASTNode::FnCall(s) => {
                    ops.push(Op::FN(s.to_string()));
                }
                ASTNode::Reset => {
                    ops.push(Op::RESET);
                }
                _ => {}
            }
        }
        Ok((ops, memo))
    }
}

pub struct Environment<'src> {
    memory: HashMap<&'src str, Mathrai>,
    pub running_count: ExecutionResult,
    pub nadai: Mathrai,
    talam: Option<usize>,
}
impl<'src> Environment<'src> {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            running_count: ExecutionResult::new(
                Mathrai(0),
                StandardAkshara::from_mathrai(Mathrai(0), Mathrai(4)),
            ),
            nadai: Mathrai(4),
            talam: None,
        }
    }
    pub fn ingest_memory(&mut self, mem: HashMap<&'src str, Mathrai>) {
        self.memory.extend(mem);
    }
    pub fn execute(&mut self, ops: Vec<Op>) -> Result<(ExecutionResult, Vec<String>), Error<'src>> {
        let mut tape = ops.iter();
        let mut aks = StandardAkshara {
            count: 0,
            edam: Carry { num: 0, den: 4 },
        };
        let mut mat = Mathrai(0);
        let mut fields: Vec<String> = vec![];
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
                    self.chk_tal_def()?;
                    mat = mat + *u;
                    aks = aks + StandardAkshara::from_mathrai(*u, self.nadai)
                }
                Op::FN(s) => {
                    self.chk_tal_def()?;
                    if let Some(u) = self.memory.get(s.as_str()) {
                        mat = mat + *u;
                        aks = aks + StandardAkshara::from_mathrai(*u, self.nadai)
                    } else {
                        return Err(Error::environment(format!("undefined fn: {}", s)));
                    }
                }
                Op::INSPECT(s) => {
                    if let Some(u) = self.memory.get(s.as_str()) {
                        fields.push(format!("{s}: {u}"));
                    }
                }
                Op::RESET => {
                    self.running_count = ExecutionResult::default();
                    fields.push("reset running count".to_string());
                }
            }
        }
        return Ok((ExecutionResult::new(mat, aks), fields));
    }
    fn chk_tal_def(&self) -> Result<(), Error<'src>> {
        if self.talam.is_none() {
            return Err(Error::EnvironmentError("undefined tal".to_string()));
        }
        Ok(())
    }
}
pub struct ExecutionResult {
    mat: Mathrai,
    aks: StandardAkshara,
}
impl ExecutionResult {
    pub fn new(mat: Mathrai, aks: StandardAkshara) -> Self {
        Self { mat, aks }
    }
    pub fn add(&mut self, rhs: Self) {
        self.mat = self.mat + rhs.mat;
        self.aks = self.aks.clone() + rhs.aks;
    }
}
impl Default for ExecutionResult {
    fn default() -> Self {
        Self {
            mat: Mathrai(0),
            aks: StandardAkshara::from_mathrai(Mathrai(0), Mathrai(4)),
        }
    }
}

impl Display for ExecutionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.mat, self.aks)
    }
}
