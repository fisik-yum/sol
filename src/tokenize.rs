#[allow(unused_imports)]
use std::{
    io::{BufReader, Read},
    iter::Peekable,
};

pub enum Token {
    SeqKw,           // keyword `seq`
    SolKw,           // keyword `sol` (holds root program)
    Literal(String), // a name
    Figure(usize),      //
    SeqStart,        // {
    SeqEnd,          // }
    GapStart,        // (
    GapEnd,          // )
    EOF,             // EOF
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "token ");
        match self {
            Self::SeqKw => write!(f, "seq"),
            Self::SolKw => write!(f, "sol"),
            Self::SeqStart => write!(f, "seq start"),
            Self::SeqEnd => write!(f, "seq end"),
            Self::Literal(s) => write!(f, "ident({})", s),
            Self::Figure(u) => write!(f, "figure({})", u),
            Self::GapStart => write!(f, "gap start"),
            Self::GapEnd => write!(f, "gap end"),
            Self::EOF => write!(f, "EOF"),
        }
    }
}
pub struct Tokenizer {
    reader: Peekable<std::vec::IntoIter<char>>,
}

impl From<String> for Tokenizer {
    fn from(value: String) -> Self {
        // dogshit code that will definitely not be fixed
        Tokenizer {
            reader: value.chars().collect::<Vec<char>>().into_iter().peekable(),
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let iter = &mut self.reader;
        if iter.peek().is_none() {
            return None;
        }
        // skip WS
        while let Some(&c) = iter.peek() {
            if c.is_whitespace() {
                iter.next();
            } else {
                break;
            }
        }
        // consume till whitespace
        if let Some(&c) = iter.peek() {
            if is_control_character(&c) {
                iter.next();
                return match_control_token(&c);
            } else if c.is_alphanumeric() {
                return extract_syntax_token(iter.by_ref());
            }
        }
        return Some(Token::EOF);
    }
}

fn is_control_character(c: &char) -> bool {
    matches!(c, '{' | '}' | '(' | ')')
}

fn match_control_token(c: &char) -> Option<Token> {
    match c {
        '{' => Some(Token::SeqStart),
        '}' => Some(Token::SeqEnd),
        '(' => Some(Token::GapStart),
        ')' => Some(Token::GapEnd),
        _ => None,
    }
}

fn extract_syntax_token(iter: &mut Peekable<std::vec::IntoIter<char>>) -> Option<Token> {
    // do we *have* to do this?
    let mut val = String::new();
    while let Some(&c) = iter.peek() {
        if is_control_character(&c) {
            break;
        }
        if c.is_whitespace() {
            break;
        }
        val.push(c);
        iter.next();
    }

    if let Ok(num_form) = val.parse::<usize>() {
        return Some(Token::Figure(num_form));
    }
    // for some reason this seems quite dodgy
    match val.as_str() {
        "seq" => Some(Token::SeqKw),
        "sol" => Some(Token::SolKw),
        _ => Some(Token::Literal(val)),
    }
}
