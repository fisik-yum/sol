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
pub struct ParseError {
    pos: Option<usize>,
    msg: String,
}

impl ParseError {
    fn at(pos: usize, msg: impl Into<String>) -> Self {
        Self {
            pos: Some(pos),
            msg: msg.into(),
        }
    }

    fn eof(msg: impl Into<String>) -> Self {
        Self {
            pos: None,
            msg: msg.into(),
        }
    }

    pub fn report(&self, filename: &str, src: &str) -> String {
        match self.pos {
            Some(pos) => {
                let (line, col) = Self::line_col(src, pos);
                format!("{filename}:{line}:{col}: error: {}", self.msg)
            }
            None => format!("{filename}: error: {} (at end of input)", self.msg),
        }
    }

    fn line_col(src: &str, pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (i, c) in src.char_indices() {
            if i >= pos {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
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
    pub fn parse(mut self) -> Result<(Node<'p>, SymbolTable<'p>), ParseError> {
        // builds an AST object
        let iter = self.tok_stream.by_ref();
        let mut tree = Node::new(NodeType::Root);
        let mut sym_table = SymbolTable::new();
        while let Some(span) = iter.peek() {
            let pos = span.start();
            match span.token() {
                Token::SeqKw => {
                    iter.next();
                    let n = parse_seq(iter.by_ref(), &mut self.stack)?;
                    let id = n.get_name();
                    let idx = tree.insert_node(n);
                    sym_table.insert(id, idx, pos)?;
                }
                Token::SolKw => {
                    return Err(ParseError::at(pos, "use of restricted keyword 'sol'"));
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
                    let n = parse_gap(iter.by_ref(), &mut self.stack)?;
                    tree.insert_node(n);
                }
                Token::GapEnd => {
                    return Err(ParseError::at(pos, "unexpected ')' (no matching gap)"));
                }
            }
        }
        Ok((tree, sym_table))
    }
}

fn parse_ident<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<&'a str, ParseError> {
    let span = iter
        .next()
        .ok_or_else(|| ParseError::eof("expected an identifier"))?;
    let pos = span.start();
    match span.token() {
        Token::Literal(s) => Ok(s),
        other => Err(ParseError::at(pos, format!("expected an identifier, found {other}"))),
    }
}

// calling fn provides the parent node to attach to
fn parse_seq<'a>(iter: &mut Peekable<Tokenizer<'a>>, stack: &mut FrameStack) -> Result<Node<'a>, ParseError> {
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

fn parse_gap<'a>(iter: &mut Peekable<Tokenizer<'a>>, stack: &mut FrameStack) -> Result<Node<'a>, ParseError> {
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
            Token::Literal(s) => {
                return Err(ParseError::at(pos, format!("cannot call '{s}' inside a gap")));
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
