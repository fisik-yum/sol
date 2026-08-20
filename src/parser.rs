use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Peekable;

use crate::ast::{Node, NodeType};
use crate::tokenize::{Token, Tokenizer};
use crate::warnings::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    Seq,
    Gap,
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

pub fn parse<'p>(tokenizer: Tokenizer<'p>) -> Result<(Node<'p>, SymbolTable<'p>), ParseError> {
    // builds an AST object
    let mut tok_stream = tokenizer.peekable();
    let iter = tok_stream.by_ref();
    let mut stack = FrameStack { stack: Vec::new() };
    let mut tree = Node::new(NodeType::Root);
    let mut sym_table = SymbolTable::new();

    // set default tal
    if let Some(tal_set) = iter.peek() {
        match tal_set.token() {
            Token::TalKw => {
                iter.next();
                let n = parse_tal(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            _ => {
                let pos = tal_set.start();
                return Err(ParseError::at(
                    pos,
                    "Expected tal decl at beginning of file",
                ));
            }
        }
    } else {
        // NOTE: is this normal behavior?
        return Err(ParseError::at(0, "Expected tal decl at beginning of file"));
    }

    // insert default nad node
    tree.insert_node(Node::new(NodeType::Nad(4)));
    // in the future we may want something more intelligent
    // this can be immediately overriden, btw.

    while let Some(span) = iter.peek() {
        let pos = span.start();
        match span.token() {
            Token::SeqKw => {
                iter.next();
                let n = parse_seq(iter.by_ref(), &mut stack)?;
                let id = n.get_name();
                let idx = tree.insert_node(n);
                sym_table.insert(id, idx, pos)?;
            }
            Token::SolKw => {
                return Err(ParseError::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::TalKw => {
                iter.next();
                let n = parse_tal(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            Token::NadKw => {
                iter.next();
                let n = parse_nad(iter.by_ref())?;
                let _ = tree.insert_node(n);
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
                return Err(ParseError::at(pos, "unexpected '{' (anonymous sequence)"));
            }
            Token::SeqEnd => {
                return Err(ParseError::at(pos, "unexpected '}' (no matching sequence)"));
            }
            Token::GapStart => {
                let n = parse_gap(iter.by_ref(), &mut stack)?;
                tree.insert_node(n);
            }
            Token::GapEnd => {
                return Err(ParseError::at(pos, "unexpected ')' (no matching gap)"));
            }
        }
    }
    Ok((tree, sym_table))
}

fn parse_ident<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<&'a str, ParseError> {
    let span = iter
        .next()
        .ok_or_else(|| ParseError::eof("expected an identifier"))?;
    let pos = span.start();
    match span.token() {
        Token::Literal(s) => Ok(s),
        other => Err(ParseError::at(
            pos,
            format!("expected an identifier, found {other}"),
        )),
    }
}
fn parse_figure<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<usize, ParseError> {
    let span = iter
        .next()
        .ok_or_else(|| ParseError::eof("expected a figure"))?;
    let pos = span.start();
    match span.token() {
        Token::Figure(u) => Ok(*u),
        other => Err(ParseError::at(
            pos,
            format!("expected an figure, found {other}"),
        )),
    }
}

fn parse_tal<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<Node<'a>, ParseError> {
    let u = parse_figure(iter)?;
    return Ok(Node::new(NodeType::Tal(u)));
}

fn parse_nad<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<Node<'a>, ParseError> {
    let u = parse_figure(iter)?;
    return Ok(Node::new(NodeType::Nad(u)));
}
// calling fn provides the parent node to attach to
fn parse_seq<'a>(
    iter: &mut Peekable<Tokenizer<'a>>,
    stack: &mut FrameStack,
) -> Result<Node<'a>, ParseError> {
    let name = parse_ident(iter.by_ref())?;
    let mut ret = Node::new(NodeType::Sequence(name));
    let sp1 = iter
        .next()
        .ok_or_else(|| ParseError::eof("expected '{' after sequence name"))?;
    match sp1.token() {
        Token::SeqStart => {
            stack.add_frame(Frame::Seq);
        }
        other => {
            return Err(ParseError::at(
                sp1.start(),
                format!("expected '{{' after sequence name, found {other}"),
            ));
        }
    };
    while let Some(span) = iter.peek() {
        let pos = span.start();
        match span.token() {
            Token::SeqKw => {
                return Err(ParseError::at(pos, "cannot nest a sequence definition"));
            }
            Token::SolKw => {
                return Err(ParseError::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::TalKw => {
                return Err(ParseError::at(pos, "Cannot invoke 'tal' inside a sequence"));
            }
            Token::NadKw => {
                return Err(ParseError::at(pos, "Cannot invoke 'nad' inside a sequence"));
            }
            Token::Literal(s) => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot call '{s}' inside a sequence (not yet supported)"),
                ));
            }
            Token::Figure(u) => {
                ret.insert_node(Node::new(NodeType::Figure(*u)));
                iter.next();
            }
            Token::SeqStart => {
                return Err(ParseError::at(pos, "unexpected '{' (anonymous sequence)"));
            }
            Token::SeqEnd => {
                iter.next();
                if stack.match_last(&Frame::Seq) {
                    stack.pop_last();
                    break;
                } else {
                    return Err(ParseError::at(pos, "unmatched '}'"));
                }
            }
            Token::GapStart => {
                let n = parse_gap(iter.by_ref(), stack)?;
                ret.insert_node(n);
            }
            Token::GapEnd => {
                return Err(ParseError::at(pos, "unexpected ')' (no matching gap)"));
            }
        }
    }
    Ok(ret)
}

fn parse_gap<'a>(
    iter: &mut Peekable<Tokenizer<'a>>,
    stack: &mut FrameStack,
) -> Result<Node<'a>, ParseError> {
    let mut ret = Node::new(NodeType::Gap);
    let sp1 = iter
        .next()
        .ok_or_else(|| ParseError::eof("expected '(' to start a gap"))?;
    match sp1.token() {
        Token::GapStart => {
            stack.add_frame(Frame::Gap);
        }
        other => {
            return Err(ParseError::at(
                sp1.start(),
                format!("expected '(' to start a gap, found {other}"),
            ));
        }
    };
    while let Some(span) = iter.peek() {
        let pos = span.start();
        match span.token() {
            Token::SeqKw => {
                return Err(ParseError::at(pos, "cannot nest a sequence definition"));
            }
            Token::SolKw => {
                return Err(ParseError::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::TalKw => {
                return Err(ParseError::at(pos, "cannot invoke 'tal' inside a gap."));
            }
            Token::NadKw => {
                return Err(ParseError::at(pos, "Cannot invoke 'nad' inside a sequence"));
            }
            Token::Literal(s) => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot call '{s}' inside a gap"),
                ));
            }
            Token::Figure(u) => {
                ret.insert_node(Node::new(NodeType::Figure(*u)));
                iter.next();
            }
            Token::SeqStart => {
                return Err(ParseError::at(pos, "unexpected '{' (anonymous sequence)"));
            }
            Token::SeqEnd => {
                return Err(ParseError::at(pos, "unopened sequence block"));
            }
            Token::GapStart => {
                return Err(ParseError::at(pos, "cannot nest gaps"));
            }
            Token::GapEnd => {
                iter.next();
                if stack.match_last(&Frame::Gap) {
                    stack.pop_last();
                    break;
                } else {
                    return Err(ParseError::at(pos, "unmatched ')'"));
                }
            }
        }
    }
    Ok(ret)
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
    pub fn insert(&mut self, k: &'a str, idx: usize, pos: usize) -> Result<(), ParseError> {
        if self.table.contains_key(k) {
            return Err(ParseError::at(pos, format!("sequence redefined: {k}")));
        }
        self.table.insert(k, idx);
        Ok(())
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
