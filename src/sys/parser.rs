use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Peekable;

use crate::sys::ast::{Node, NodeType};
use crate::sys::tokenize::{Token, Tokenizer};
use crate::sys::warnings::ParseError;

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
            Token::Tal => {
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
            Token::Seq => {
                iter.next();
                let n = parse_seq(iter.by_ref(), &mut stack)?;
                let id = n.get_name();
                let idx = tree.insert_node(n);
                sym_table.insert(id, idx, pos)?;
            }
            Token::Sol => {
                return Err(ParseError::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::Tal => {
                iter.next();
                let n = parse_tal(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            Token::Nad => {
                iter.next();
                let n = parse_nad(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            Token::Mat => {
                iter.next();
                let n = parse_mat(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            // TODO
            Token::Aks => {
                iter.next();
                let n = parse_aks(iter.by_ref())?;
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
fn parse_expect_sol<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<bool, ParseError> {
    let span = iter.next().ok_or_else(|| ParseError::eof("expected sol"))?;
    let pos = span.start();
    println!("{}", span.token());
    match span.token() {
        Token::Sol => Ok(true),
        other => Err(ParseError::at(pos, format!("expected sol, found {other}"))),
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

fn parse_mat<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<Node<'a>, ParseError> {
    // certainly a bad bit of code
    let pos = iter
        .peek()
        .ok_or_else(|| ParseError::eof("expected identifier"))?
        .start();
    match iter.next().unwrap().token() {
        Token::Literal(u) => Ok(Node::new(NodeType::MatLocal(u))),
        Token::Sol => Ok(Node::new(NodeType::MatGlobal)),
        _ => Err(ParseError::at(
            pos,
            "expected either an identifier or global keyword 'sol' as argument",
        )),
    }
}
fn parse_aks<'a>(_iter: &mut Peekable<Tokenizer<'a>>) -> Result<Node<'a>, ParseError> {
    return Ok(Node::new(NodeType::Aks));
}
#[derive(Clone, Copy)]
enum BodyKind {
    Seq,
    Gap,
}

impl BodyKind {
    fn name(&self) -> &'static str {
        match self {
            BodyKind::Seq => "a sequence",
            BodyKind::Gap => "a gap",
        }
    }
}

fn parse_body<'a>(
    iter: &mut Peekable<Tokenizer<'a>>,
    stack: &mut FrameStack,
    kind: BodyKind,
    target: &mut Node<'a>,
) -> Result<(), ParseError> {
    while let Some(span) = iter.peek() {
        let pos = span.start();
        match span.token() {
            Token::Seq => {
                return Err(ParseError::at(pos, "cannot nest a sequence definition"));
            }
            Token::Sol => {
                return Err(ParseError::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::Tal => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot invoke 'tal' inside {}", kind.name()),
                ));
            }
            Token::Nad => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot invoke 'nad' inside {}", kind.name()),
                ));
            }
            Token::Mat => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot invoke 'mat' inside {}", kind.name()),
                ));
            }
            Token::Aks => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot invoke 'aks' inside {}", kind.name()),
                ));
            }
            Token::Literal(s) => {
                return Err(ParseError::at(
                    pos,
                    format!("cannot call '{s}' inside {}", kind.name()),
                ));
            }
            Token::Figure(u) => {
                target.insert_node(Node::new(NodeType::Figure(*u)));
                iter.next();
            }
            Token::SeqStart => {
                return Err(ParseError::at(pos, "unexpected '{' (anonymous sequence)"));
            }
            Token::SeqEnd => match kind {
                BodyKind::Seq => {
                    iter.next();
                    if stack.match_last(&Frame::Seq) {
                        stack.pop_last();
                        return Ok(());
                    } else {
                        return Err(ParseError::at(pos, "unmatched '}'"));
                    }
                }
                BodyKind::Gap => {
                    return Err(ParseError::at(pos, "unopened sequence block"));
                }
            },
            Token::GapStart => match kind {
                BodyKind::Seq => {
                    let n = parse_gap(iter.by_ref(), stack)?;
                    target.insert_node(n);
                }
                BodyKind::Gap => {
                    return Err(ParseError::at(pos, "cannot nest gaps"));
                }
            },
            Token::GapEnd => match kind {
                BodyKind::Gap => {
                    iter.next();
                    if stack.match_last(&Frame::Gap) {
                        stack.pop_last();
                        return Ok(());
                    } else {
                        return Err(ParseError::at(pos, "unmatched ')'"));
                    }
                }
                BodyKind::Seq => {
                    return Err(ParseError::at(pos, "unexpected ')' (no matching gap)"));
                }
            },
        }
    }
    Ok(())
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
    parse_body(iter, stack, BodyKind::Seq, &mut ret)?;
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
    parse_body(iter, stack, BodyKind::Gap, &mut ret)?;
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

    pub fn get(&self, k: &'a str) -> Result<usize, ParseError> {
        self.table
            .get(k)
            .copied()
            .ok_or_else(|| ParseError::global(format!("undefined sequence: {k}")))
    }
}

impl std::fmt::Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.table.fmt(f)
    }
}
