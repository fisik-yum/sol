use std::str::CharIndices;
#[allow(unused_imports)]
use std::{
    io::{BufReader, Read},
    iter::Peekable,
};

pub enum Token<'t> {
    Seq,            // keyword `seq`
    Sol,            // keyword `sol` (holds root program)
    Tal,            // keyword `tal`
    Nad,            // keyword `nad`
    Mat,            // keyword 'mat'
    Aks,            // keyword 'aks'
    Literal(&'t str), // a name
    Figure(usize),    //
    SeqStart,         // {
    SeqEnd,           // }
    GapStart,         // (
    GapEnd,           // )
}

pub struct SpanToken<'s> {
    start: usize,
    tok: Token<'s>,
}
impl<'s> SpanToken<'s> {
    pub fn new(t: Token<'s>, s: usize) -> Self {
        Self { start: s, tok: t }
    }

    pub fn token(&self) -> &Token<'s> {
        &self.tok
    }

    pub fn start(&self) -> usize {
        self.start
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "token ");
        match self {
            Self::Seq => write!(f, "seq"),
            Self::Sol => write!(f, "sol"),
            Self::Tal => write!(f, "tal"),
            Self::Nad => write!(f, "nad"),
            Self::Mat=> write!(f, "mat"),
            Self::Aks=> write!(f, "aks"),
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
    type Item = SpanToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let (start, first_char) = *self.chars.peek()?;

        if is_control_character(first_char) {
            self.chars.next(); // Consume '(' or ')' or '{' or '}'
            return Some(SpanToken::new(match_control_token(first_char), start));
        }

        let mut end = self.source.len();
        while let Some(&(i, c)) = self.chars.peek() {
            if c.is_whitespace() || is_control_character(c) {
                end = i;
                break;
            }
            self.chars.next();
        }

        let slice = &self.source[start..end];
        return Some(SpanToken::new(match_syntax_token(slice), start));
    }
}
fn match_control_token<'a>(c: char) -> Token<'a> {
    match c {
        '{' => Token::SeqStart,
        '}' => Token::SeqEnd,
        '(' => Token::GapStart,
        ')' => Token::GapEnd,
        _ => panic!("stoopid edge case!"),
    }
}
fn is_control_character(c: char) -> bool {
    matches!(c, '{' | '}' | '(' | ')')
}
fn match_syntax_token<'a>(val: &'a str) -> Token<'a> {
    if let Ok(num_form) = val.parse::<usize>() {
        return Token::Figure(num_form);
    }
    // for some reason this seems quite dodgy
    match val {
        "sol" => Token::Sol,
        "seq" => Token::Seq,
        "tal" => Token::Tal,
        "nad" => Token::Nad,
        "mat" => Token::Mat,
        "aks" => Token::Aks,
        _ => Token::Literal(val),
    }
}
