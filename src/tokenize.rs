use std::{char, str::CharIndices};
#[allow(unused_imports)]
use std::{
    io::{BufReader, Read},
    iter::Peekable,
};

pub enum Token<'a> {
    SeqKw,            // keyword `seq`
    SolKw,            // keyword `sol` (holds root program)
    Literal(&'a str), // a name
    Figure(usize),    //
    SeqStart,         // {
    SeqEnd,           // }
    GapStart,         // (
    GapEnd,           // )
}
impl std::fmt::Display for Token<'_> {
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
        }
    }
}
pub struct Tokenizer<'a> {
    chars: Peekable<CharIndices<'a>>,
    source: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        // dogshit code that will definitely not be fixed
        Tokenizer {
            chars: value.char_indices().peekable(),
            source: value,
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let (start, _) = *self.chars.peek()?;
        let mut end = self.source.len();
        while let Some(&(i, c)) = self.chars.peek() {
            if c.is_whitespace() {
                end = i;
                break;
            } else if is_control_character(&c) {
                self.chars.next();
                return match_control_token(c);
            }
            self.chars.next();
        }
        let slice = &self.source[start..end];
        return extract_syntax_token(slice);
    }
}

fn is_control_character(c: &char) -> bool {
    matches!(c, '{' | '}' | '(' | ')')
}

fn match_control_token<'a>(c: char) -> Option<Token<'a>> {
    match c {
        '{' => Some(Token::SeqStart),
        '}' => Some(Token::SeqEnd),
        '(' => Some(Token::GapStart),
        ')' => Some(Token::GapEnd),
        _ => None,
    }
}

fn extract_syntax_token(val: &str) -> Option<Token> {
    if let Ok(num_form) = val.parse::<usize>() {
        return Some(Token::Figure(num_form));
    }
    // for some reason this seems quite dodgy
    match val {
        "seq" => Some(Token::SeqKw),
        "sol" => Some(Token::SolKw),
        _ => Some(Token::Literal(val)),
    }
}
