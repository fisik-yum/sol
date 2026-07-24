use std::{io::BufReader, iter::Peekable};

pub enum Token {
    SeqKw,         // keyword `seq`
    Ident(String), // a name
    SeqStart,      // {
    SeqEnd,        // }
    GapStart,      // (
    GapEnd,        // )
    WS,            // whitespace
    EOF,           // EOF
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "token ");
        match self {
            Self::SeqKw => write!(f, "seq"),
            Self::SeqStart => write!(f, "seq start"),
            Self::SeqEnd => write!(f, "seq end"),
            Self::Ident(s) => write!(f, "ident({})", s),
            Self::GapStart => write!(f, "gap start"),
            Self::GapEnd => write!(f, "gap end"),
            Self::WS => write!(f, "whitespace"),
            Self::EOF => write!(f, "EOF"),
        }
    }
}
pub struct Tokenizer {
    reader: Peekable<std::vec::IntoIter<char>>,
    eof: bool,
}

impl From<String> for Tokenizer {
    fn from(s: String) -> Self {
        // dogshit code that will definitely not be fixed
        Tokenizer {
            reader: s.chars().collect::<Vec<char>>().into_iter().peekable(),
            eof: false,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;
    // NOTE: for now, skip tokenizing whitespace.
    // this may change
    fn next(&mut self) -> Option<Self::Item> {
        let iter = &mut self.reader;
        if self.eof == true{
            return None;
        }
        if iter.peek().is_none() {
            self.eof = true;
            return Some(Token::EOF);
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
        let mut token_string = String::from("");
        while let Some(&c) = iter.peek() {
            if !c.is_whitespace() {
                token_string.push(c);
                iter.next();
            } else {
                break;
            }
        }
        return Some(match_token(token_string));
    }
}

fn match_token(s: String) -> Token {
    match s.as_str() {
        "seq" => Token::SeqKw,
        "{" => Token::SeqStart,
        "}" => Token::SeqEnd,
        "(" => Token::GapStart,
        ")" => Token::GapEnd,
        _ => Token::Ident(s),
    }
}
