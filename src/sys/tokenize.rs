use std::str::CharIndices;
#[allow(unused_imports)]
use std::{
    io::{BufReader, Read},
    iter::Peekable,
};
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    Seq,     // keyword `seq`
    Sol,     // keyword `sol` (holds root program)
    Tal,     // keyword `tal`
    Nad,     // keyword `nad`
    Literal, // a name
    Figure,  //
    Control, // )
    // "introspection" codes for interactive mode
    Load,
    Reload,
    Inspect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span<'s> {
    start: usize,
    end: usize,
    source: &'s str,
    token: Token,
}
impl<'s> Span<'s> {
    pub fn new(token: Token, start: usize, end: usize, source: &'s str) -> Self {
        Self {
            start,
            end,
            source,
            token,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn start(&self) -> usize {
        self.start
    }
    pub fn text(&self) -> &'s str {
        &self.source[self.start..self.end]
    }
    pub fn is(&self, token: Token) -> bool {
        return self.token == token;
    }
    pub fn to_line(&self) -> (usize, usize) {
        let mut row: usize = 1;
        let mut col: usize = 0;
        for (_, c) in self.source[..self.start + 1].char_indices() {
            if c == '\n' || c == '\r' {
                row += 1;
                col = 0;
            }
            col += 1;
        }
        (row, col)
    }
}

// TODO: move this implementation to Span
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "token ");
        match self {
            Self::Seq => write!(f, "seq"),
            Self::Sol => write!(f, "sol"),
            Self::Tal => write!(f, "tal"),
            Self::Nad => write!(f, "nad"),
            Self::Literal => write!(f, "ident()",),
            Self::Figure => write!(f, "figure()"),
            Self::Control => write!(f, "control"),

            Self::Inspect => write!(f, "inspect"),
            Self::Load => write!(f, "load"),
            Self::Reload => write!(f, "reload"),
        }
    }
}

#[derive(Debug, Clone)]
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
impl<'a> From<&'a String> for Tokenizer<'a> {
    fn from(value: &'a String) -> Self {
        Tokenizer {
            chars: value.char_indices().peekable(),
            source: value.as_str(),
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
    fn skip_comments(&mut self) -> bool {
        if let Some(&(i, _)) = self.chars.peek() {
            if self.source[i..].starts_with("//") {
                self.chars.next();
                self.chars.next();
                while let Some(&(_, c)) = self.chars.peek() {
                    if c == '\n' || c == '\r' {
                        self.chars.next();
                        break;
                    } else {
                        self.chars.next();
                    }
                }
                return true;
            }
        }
        false
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.skip_whitespace();
            if !self.skip_comments() {
                break;
            }
        }

        let (start, first_char) = *self.chars.peek()?;

        if is_control_character(first_char) {
            self.chars.next(); // Consume '(' or ')' or '{' or '}'
            return Some(Span::new(Token::Control, start, start + 1, self.source));
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
        return Some(Span::new(
            match_syntax_token(slice),
            start,
            end,
            self.source,
        ));
    }
}

fn is_control_character(c: char) -> bool {
    matches!(c, '{' | '}' | '(' | ')')
}
fn match_syntax_token<'a>(val: &'a str) -> Token {
    if let Ok(_) = val.parse::<usize>() {
        return Token::Figure;
    }
    // for some reason this seems quite dodgy
    match val {
        "sol" => Token::Sol,
        "seq" => Token::Seq,
        "tal" => Token::Tal,
        "nad" => Token::Nad,
        "rld" => Token::Reload,
        "ins" => Token::Inspect,
        "lod" => Token::Load,
        _ => Token::Literal,
    }
}
