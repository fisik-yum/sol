use std::iter::Peekable;

use crate::ast::{Node, NodeType};
use crate::tokenize::{Token, Tokenizer};

#[allow(dead_code)]
enum State {
    Block(bool),
    Gap(bool),
    Hold,
}

#[allow(dead_code)]
pub struct Parser {
    tok_stream: Peekable<Tokenizer>,
    stack: Vec<State>,
}

impl From<Tokenizer> for Parser {
    fn from(value: Tokenizer) -> Self {
        Self {
            tok_stream: value.peekable(),
            stack: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl Parser {
    fn parse(&mut self) -> Node {
        // builds an AST object
        let iter = self.tok_stream.by_ref();
        let mut tree = Node::new(NodeType::Root);
        while let Some(tok) = iter.peek() {
            match tok {
                Token::SeqKw => {
                    iter.next();

                    let seq_name = match iter.next() {
                        Some(Token::Literal(s)) => s,
                        _ => panic!("malformed seq declaration"),
                    };
                }
                Token::SolKw => {}
                Token::Literal(s) => {
                    let insertion = Node::new(NodeType::FnCall(s.to_owned()));
                    if let Some(ch) = &mut tree.children {
                        ch.push(insertion);
                    }
                    iter.next();
                }
                Token::Figure(u) => {
                    let insertion = Node::new(NodeType::Figure(*u));
                    if let Some(ch) = &mut tree.children {
                        ch.push(insertion);
                    }
                    iter.next();
                }
                Token::EOF => {}
                Token::SeqStart => {}
                Token::SeqEnd => {}
                Token::GapStart => {}
                Token::GapEnd => {}
            }
        }
        tree
    }

    // calling fn provides the parent node to attach to
    fn parse_inner(&mut self) -> Node {
        return Node::new(NodeType::Root);
    }

    fn add_state(&mut self, state: State) -> Result<(), ()> {
        if let Some(last) = self.stack.last() {
        } else {
        }

        self.stack.push(state);
        Ok(())
    }
}
