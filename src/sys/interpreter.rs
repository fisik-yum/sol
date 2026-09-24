use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use talm::aks::{Carry, StandardAkshara};
use talm::unit::Mathrai;

use crate::sys::ast::ASTNode;
use crate::sys::warnings::Error;
use crate::sys::{stdlib, transforms};

pub struct InterpreterResult {
    total_mat_count: Mathrai,
    total_aks_count: StandardAkshara,
}
impl Display for InterpreterResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.total_mat_count, self.total_aks_count)
    }
}

pub struct Environment<'p> {
    stages: Pipeline<'p>,
    pub sym_table: SymbolTable<'p>,
}

impl<'p> Environment<'p> {
    pub fn new(p: Pipeline<'p>) -> Self {
        Self {
            stages: p,
            sym_table: SymbolTable::default(),
        }
    }
    pub fn interpret(&mut self, mut root: ASTNode<'p>) -> Result<InterpreterResult, Error> {
        root = self.stages.ingest(root)?;
        let result = interpret(&root, &mut self.sym_table)?;
        Ok(result)
    }
}

/*
 * the interpreter sequentially builds the symbol table as it evaluates
 * a program.
 * The symbol table is a shared state of the interpreter environment.
 */
fn interpret<'p>(
    root: &ASTNode<'p>,
    symbols: &mut SymbolTable<'p>,
) -> Result<InterpreterResult, Error> {
    let children = root.get_children();

    // interpreter state
    let mut memo: HashMap<&str, Mathrai> = HashMap::new();
    // program parameters; default nadai is 4 (by convention)
    let mut talam: Option<usize> = None;
    let mut nadai = Mathrai(4);
    // accumulators
    let mut mat_count = Mathrai(0);
    let mut aks_count = StandardAkshara {
        count: 0,
        edam: Carry {
            num: 0,
            den: nadai.0,
        },
    };

    for child in children {
        match &(**child) {
            ASTNode::Tal(u) => {
                if talam.is_none() {
                    talam = Some(*u);
                } else {
                    return Err(Error::global("cannot redeclare tal param"));
                }
            }
            ASTNode::Nad(u) => {
                nadai = Mathrai(*u);
            }
            ASTNode::Sequence(s, _) => {
                symbols.insert(s, Rc::clone(child))?;
            }
            ASTNode::Figure(u) => {
                mat_count = mat_count + Mathrai(*u);
                aks_count = aks_count + StandardAkshara::from_mathrai(Mathrai(*u), nadai);
            }
            ASTNode::Gap(v) => {
                if talam.is_none() {
                    return Err(Error::global("tal param left undeclared"));
                }
                let mut count = Mathrai(0);
                for fig in v {
                    match **fig {
                        ASTNode::Figure(u) => {
                            count = count + Mathrai(u);
                        }
                        _ => return Err(Error::global("some unidentified error")),
                    }
                }
                mat_count = mat_count + count;
                aks_count = aks_count + StandardAkshara::from_mathrai(count, nadai);
            }
            ASTNode::FnCall(f) => {
                if talam.is_none() {
                    return Err(Error::global("tal param left undeclared"));
                }
                let count: Mathrai;
                if memo.contains_key(f) {
                    count = memo.get(f).unwrap().clone();
                } else {
                    let seq = symbols.get(f)?;
                    count = stdlib::mat::seq_count_m(&seq)?;
                    memo.insert(f, count);
                }
                mat_count = mat_count + count;
                aks_count = aks_count + StandardAkshara::from_mathrai(count, nadai);
            }
            _ => {}
        }
    }
    let result = InterpreterResult {
        total_mat_count: mat_count,
        total_aks_count: aks_count,
    };
    Ok(result)
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

    pub fn ingest(&self, mut input: ASTNode<'p>) -> Result<ASTNode<'p>, Error> {
        for stage in self.transform_path.as_slice() {
            input = stage.mutate(input).unwrap();
        }
        Ok(input)
    }
}

pub struct SymbolTable<'p> {
    table: HashMap<&'p str, Rc<ASTNode<'p>>>,
}

impl<'p> SymbolTable<'p> {
    pub fn new(root: &ASTNode<'p>) -> Result<Self, Error> {
        let mut ret = Self {
            table: HashMap::new(),
        };

        let children = root.get_children().as_slice();

        for i in 0..children.len() {
            let child = &children[i];
            match **child {
                ASTNode::Sequence(s, _) => {
                    ret.insert(s, Rc::clone(child))?;
                }
                _ => {}
            };
        }
        Ok(ret)
    }
    pub fn insert(&mut self, k: &'p str, node: Rc<ASTNode<'p>>) -> Result<(), Error> {
        if self.table.contains_key(k) {
            return Err(Error::global(format!("sequence redefined: {k}")));
        }
        self.table.insert(k, node);
        Ok(())
    }

    pub fn get(&self, k: &str) -> Result<Rc<ASTNode<'p>>, Error> {
        self.table
            .get(k)
            .cloned()
            .ok_or_else(|| Error::global(format!("undefined sequence: {k}")))
    }
}

impl<'s> Default for SymbolTable<'s> {
    fn default() -> Self {
        return Self {
            table: HashMap::new(),
        };
    }
}

impl std::fmt::Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.table.fmt(f)
    }
}
