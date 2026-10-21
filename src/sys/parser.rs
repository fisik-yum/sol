use std::fmt::Debug;
use std::iter::Peekable;
use std::rc::Rc;

use crate::sys::ast::ASTNode;
use crate::sys::tokenize::{Token, Tokenizer};
use crate::sys::warnings::Error;

pub fn parse<'p>(tokenizer: Tokenizer<'p>) -> Result<ASTNode<'p>, Error> {
    // builds an AST object
    let mut tok_stream = tokenizer.peekable();
    let iter = tok_stream.by_ref();
    let mut tree = ASTNode::Root(vec![]);

    while let Some(span) = iter.peek() {
        match span.token() {
            Token::Tal => {
                tree.insert_node(parse_tal(iter)?);
            }
            Token::Nad => {
                tree.insert_node(parse_nad(iter)?);
            }
            Token::Seq => {
                tree.insert_node(parse_seq(iter)?);
            }
            Token::Literal => {
                tree.insert_node(parse_fcl(iter)?);
            }
            Token::Figure => {
                tree.insert_node(parse_fig(iter)?);
            }
            Token::Load => {
                tree.insert_node(parse_lod(iter)?);
            }
            Token::Inspect => {
                tree.insert_node(parse_ins(iter)?);
            }
            Token::Reload => {
                tree.insert_node(ASTNode::Reload);
            }
            Token::Control => {
                // TODO: throw errors
                iter.next();
            }
            _ => {}
        }
    }
    Ok(tree)
}

fn parse_tal<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    iter.next();
    if let Some(span) = iter.next() {
        let val: usize = span.text().parse().unwrap(); // TODO: remove unwrap
        return Ok(ASTNode::Tal(val));
    }
    panic!()
}
fn parse_nad<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    iter.next();
    if let Some(span) = iter.next() {
        let val: usize = span.text().parse().unwrap(); // TODO: remove unwrap
        return Ok(ASTNode::Nad(val));
    }
    panic!()
}

fn parse_fig<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    if let Some(span) = iter.next() {
        let val: usize = span.text().parse().unwrap(); // TODO: remove unwrap
        return Ok(ASTNode::Figure(val));
    }
    panic!()
}
fn parse_fcl<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    if let Some(span) = iter.next() {
        let val: &str = span.text();
        return Ok(ASTNode::FnCall(val));
    }
    panic!()
}
fn parse_lod<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    iter.next();
    if let Some(span) = iter.next() {
        let val: &str = span.text();
        return Ok(ASTNode::Load(val));
    }
    panic!()
}
fn parse_ins<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    iter.next();
    if let Some(span) = iter.next() {
        let val: &str = span.text();
        return Ok(ASTNode::Inspect(val));
    }
    panic!()
}
fn parse_seq<'p>(iter: &mut Peekable<Tokenizer<'p>>) -> Result<ASTNode<'p>, Error> {
    iter.next();
    // TODO: reorg into less bp
    if let Some(mark) = iter.next_if(|x| x.is(Token::Literal)) {
        let mut seq = ASTNode::Sequence(mark.text(), vec![]);
        if let Some(_) = iter.next_if(|x| x.is(Token::Control) && x.text() == "{") {
            while let Some(span) = iter.peek() {
                if span.text() == "}" && span.is(Token::Control) {
                    iter.next();
                    return Ok(seq);
                }else if span.text() == "{" && span.is(Token::Control) {
                    return Err(Error::at(span.start(), "wrong control token"));
                }
                match span.token() {
                    Token::Figure => {
                        seq.insert_node(parse_fig(iter)?);
                    }
                    Token::Control => {
                        // TODO: throw errors
                        iter.next();
                    }
                    _ => {}
                }
            }
        }
    } 
    panic!()
}
