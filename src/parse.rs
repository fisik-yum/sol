use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Peekable;

use crate::ast::{Node, NodeType};
use crate::tokenize::{Token, Tokenizer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    Seq,
    Gap,
}

#[allow(dead_code)]
pub struct Parser<'p> {
    tok_stream: Peekable<Tokenizer<'p>>,
    stack: FrameStack,
}

pub struct FrameStack {
    stack: Vec<Frame>,
}
impl FrameStack {
    pub fn add_frame(&mut self, state: Frame) {
        self.stack.push(state);
    }

    pub fn match_last(&mut self, state: &Frame) -> bool {
        if let Some(last) = self.stack.last() {
            if last == state {
                return true;
            }
            return false;
        };
        return false;
    }

    pub fn pop_last(&mut self) {
        self.stack.pop();
    }
}

impl<'p> From<Tokenizer<'p>> for Parser<'p> {
    fn from(value: Tokenizer<'p>) -> Self {
        Self {
            tok_stream: value.peekable(),
            stack: FrameStack { stack: Vec::new() },
        }
    }
}

#[allow(dead_code)]
impl<'p> Parser<'p> {
    pub fn parse(mut self) -> (Node<'p>, SymbolTable<'p>) {
        // builds an AST object
        let iter = self.tok_stream.by_ref();
        let mut tree = Node::new(NodeType::Root);
        let mut sym_table = SymbolTable::new();
        while let Some(tok) = iter.peek() {
            match tok {
                Token::SeqKw => {
                    iter.next();
                    let seq_name = match iter.next() {
                        Some(Token::Literal(s)) => s,
                        _ => panic!("expected seq identifier"),
                    };
                    let n = parse_seq(seq_name, iter.by_ref(), &mut self.stack);
                    sym_table.insert(seq_name, tree.children.as_ref().unwrap().len());
                    tree.insert_node(n);
                }
                Token::SolKw => {
                    panic!("use of restricted keyword");
                }
                Token::Literal(s) => {
                    tree.insert_node(Node::new(NodeType::FnCall(*s)));
                    iter.next();
                }
                Token::Figure(u) => {
                    tree.insert_node(Node::new(NodeType::Figure(*u)));
                    iter.next();
                }
                Token::SeqStart => {
                    panic!("anonymous sequence defined");
                }
                Token::SeqEnd => {
                    panic!("implies anonymous sequence defined");
                }
                Token::GapStart => {
                    let n = parse_gap(iter.by_ref(), &mut self.stack);
                    tree.insert_node(n);
                }
                Token::GapEnd => {
                    panic!("implies anonymous gap defined");
                }
            }
        }
        (tree, sym_table)
    }
}
// calling fn provides the parent node to attach to
fn parse_seq<'a>(n: &'a str, iter: &mut Peekable<Tokenizer>, stack: &mut FrameStack) -> Node<'a> {
    let mut ret = Node::new(NodeType::Sequence(n));
    let _ = match iter.next() {
        Some(Token::SeqStart) => {
            stack.add_frame(Frame::Seq);
        }
        _ => panic!("expected {{"),
    };
    while let Some(tok) = iter.peek() {
        match tok {
            Token::SeqKw => {
                panic!("cannot nest sequence")
            }
            Token::SolKw => {
                panic!("use of restricted keyword");
            }
            Token::Literal(s) => {
                // iter.next();
                panic!("WIP: cannot FnCall inside sequence {s}");
            }
            Token::Figure(u) => {
                ret.insert_node(Node::new(NodeType::Figure(*u)));
                iter.next();
            }
            Token::SeqStart => {
                panic!("anonymous sequence defined");
            }
            Token::SeqEnd => {
                iter.next();
                if stack.match_last(&Frame::Seq) {
                    stack.pop_last();
                    break;
                } else {
                    // why ?? again?
                    panic!("implies anonymous sequence")
                }
            }
            Token::GapStart => {
                let n = parse_gap(iter.by_ref(), stack);
                ret.insert_node(n);
            }
            Token::GapEnd => {
                panic!("unexpected gap end")
            }
        }
    }
    return ret;
}

fn parse_gap<'a>(iter: &mut Peekable<Tokenizer>, stack: &mut FrameStack) -> Node<'a> {
    let mut ret = Node::new(NodeType::Gap);
    let _ = match iter.next() {
        Some(Token::GapStart) => {
            stack.add_frame(Frame::Gap);
        }
        _ => panic!("expected {{"),
    };
    while let Some(tok) = iter.peek() {
        match tok {
            Token::SeqKw => {
                panic!("cannot nest sequence")
            }
            Token::SolKw => {
                panic!("use of restricted keyword");
            }
            Token::Literal(_) => {
                // iter.next();
                panic!("cannot FnCall inside gap");
            }
            Token::Figure(u) => {
                ret.insert_node(Node::new(NodeType::Figure(*u)));
                iter.next();
            }
            Token::SeqStart => {
                panic!("anonymous sequence defined");
            }
            Token::SeqEnd => {
                panic!("unopened sequence block");
            }
            Token::GapStart => {
                panic!("cannot nest gaps");
            }
            Token::GapEnd => {
                iter.next();
                if stack.match_last(&Frame::Gap) {
                    stack.pop_last();
                    break;
                } else {
                    // why ?? again?
                    panic!("implies anonymous gap")
                }
            }
        }
    }
    return ret;
}
pub struct SymbolTable<'a> {
    pub table: HashMap<&'a str, usize>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    pub fn insert(&mut self, k: &'a str, idx: usize) {
        if self.table.contains_key(k) {
            panic!("sequence redefined: {k}")
        } else {
            self.table.insert(k, idx);
        }
    }

    pub fn get(&self, k: &'a str) -> usize {
        if self.table.contains_key(k) {
            return *self.table.get(k).unwrap();
        }
        panic!("undefined sequence")
    }
}

impl std::fmt::Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.table.fmt(f)
    }
}
