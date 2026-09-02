use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Peekable;

use crate::sys::ast::ASTNode;
use crate::sys::tokenize::{Token, Tokenizer};
use crate::sys::warnings::Error;
use crate::sys::{Program, SymbolTable};

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

pub fn parse<'p>(tokenizer: Tokenizer<'p>) -> Result<Program<'p>, Error> {
    // builds an AST object
    let mut tok_stream = tokenizer.peekable();
    let iter = tok_stream.by_ref();
    let mut stack = FrameStack { stack: Vec::new() };
    let mut tree = ASTNode::Root(vec![]);
    let mut sym_table = SymbolTable::new();

    // set default tal
    let u: usize;
    if let Some(tal_set) = iter.peek() {
        match tal_set.token() {
            Token::Tal => {
                iter.next();
                let (n, k) = parse_tal(iter.by_ref())?;
                u = k;
                let _ = tree.insert_node(n);
            }
            _ => {
                let pos = tal_set.start();
                return Err(Error::at(pos, "Expected tal decl at beginning of file"));
            }
        }
    } else {
        // NOTE: is this normal behavior?
        return Err(Error::at(0, "Expected tal decl at beginning of file"));
    }

    // insert default nad node
    tree.insert_node(ASTNode::Nad(4));
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
                return Err(Error::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::Tal => {
                iter.next();
                let (n, _) = parse_tal(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            Token::Nad => {
                iter.next();
                let n = parse_nad(iter.by_ref())?;
                let _ = tree.insert_node(n);
            }
            Token::Literal(s) => {
                tree.insert_node(ASTNode::FnCall(*s));
                iter.next();
            }
            Token::Figure(u) => {
                tree.insert_node(ASTNode::Figure(*u));
                iter.next();
            }
            Token::SeqStart => {
                return Err(Error::at(pos, "unexpected '{' (anonymous sequence)"));
            }
            Token::SeqEnd => {
                return Err(Error::at(pos, "unexpected '}' (no matching sequence)"));
            }
            Token::GapStart => {
                let n = parse_gap(iter.by_ref(), &mut stack)?;
                tree.insert_node(n);
            }
            Token::GapEnd => {
                return Err(Error::at(pos, "unexpected ')' (no matching gap)"));
            }
        }
    }
    Ok(Program::new(tree, u, sym_table, HashMap::new()))
}

fn parse_ident<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<&'a str, Error> {
    let span = iter
        .next()
        .ok_or_else(|| Error::eof("expected an identifier"))?;
    let pos = span.start();
    match span.token() {
        Token::Literal(s) => Ok(s),
        other => Err(Error::at(
            pos,
            format!("expected an identifier, found {other}"),
        )),
    }
}
fn parse_figure<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<usize, Error> {
    let span = iter.next().ok_or_else(|| Error::eof("expected a figure"))?;
    let pos = span.start();
    match span.token() {
        Token::Figure(u) => Ok(*u),
        other => Err(Error::at(pos, format!("expected an figure, found {other}"))),
    }
}

fn parse_tal<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<(ASTNode<'a>, usize), Error> {
    let u = parse_figure(iter)?;
    return Ok((ASTNode::Tal(u), u));
}

fn parse_nad<'a>(iter: &mut Peekable<Tokenizer<'a>>) -> Result<ASTNode<'a>, Error> {
    let u = parse_figure(iter)?;
    return Ok(ASTNode::Nad(u));
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
    target: &mut ASTNode<'a>,
) -> Result<(), Error> {
    while let Some(span) = iter.peek() {
        let pos = span.start();
        match span.token() {
            Token::Seq => {
                return Err(Error::at(pos, "cannot nest a sequence definition"));
            }
            Token::Sol => {
                return Err(Error::at(pos, "use of restricted keyword 'sol'"));
            }
            Token::Tal => {
                return Err(Error::at(
                    pos,
                    format!("cannot invoke 'tal' inside {}", kind.name()),
                ));
            }
            Token::Nad => {
                return Err(Error::at(
                    pos,
                    format!("cannot invoke 'nad' inside {}", kind.name()),
                ));
            }
            Token::Literal(s) => {
                return Err(Error::at(
                    pos,
                    format!("cannot call '{s}' inside {}", kind.name()),
                ));
            }
            Token::Figure(u) => {
                target.insert_node(ASTNode::Figure(*u));
                iter.next();
            }
            Token::SeqStart => {
                return Err(Error::at(pos, "unexpected '{' (anonymous sequence)"));
            }
            Token::SeqEnd => match kind {
                BodyKind::Seq => {
                    iter.next();
                    if stack.match_last(&Frame::Seq) {
                        stack.pop_last();
                        return Ok(());
                    } else {
                        return Err(Error::at(pos, "unmatched '}'"));
                    }
                }
                BodyKind::Gap => {
                    return Err(Error::at(pos, "unopened sequence block"));
                }
            },
            Token::GapStart => match kind {
                BodyKind::Seq => {
                    let n = parse_gap(iter.by_ref(), stack)?;
                    target.insert_node(n);
                }
                BodyKind::Gap => {
                    return Err(Error::at(pos, "cannot nest gaps"));
                }
            },
            Token::GapEnd => match kind {
                BodyKind::Gap => {
                    iter.next();
                    if stack.match_last(&Frame::Gap) {
                        stack.pop_last();
                        return Ok(());
                    } else {
                        return Err(Error::at(pos, "unmatched ')'"));
                    }
                }
                BodyKind::Seq => {
                    return Err(Error::at(pos, "unexpected ')' (no matching gap)"));
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
) -> Result<ASTNode<'a>, Error> {
    let name = parse_ident(iter.by_ref())?;
    let mut ret = ASTNode::Sequence(name, vec![]);
    let sp1 = iter
        .next()
        .ok_or_else(|| Error::eof("expected '{' after sequence name"))?;
    match sp1.token() {
        Token::SeqStart => {
            stack.add_frame(Frame::Seq);
        }
        other => {
            return Err(Error::at(
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
) -> Result<ASTNode<'a>, Error> {
    let mut ret = ASTNode::Gap(vec![]);
    let sp1 = iter
        .next()
        .ok_or_else(|| Error::eof("expected '(' to start a gap"))?;
    match sp1.token() {
        Token::GapStart => {
            stack.add_frame(Frame::Gap);
        }
        other => {
            return Err(Error::at(
                sp1.start(),
                format!("expected '(' to start a gap, found {other}"),
            ));
        }
    };
    parse_body(iter, stack, BodyKind::Gap, &mut ret)?;
    Ok(ret)
}
