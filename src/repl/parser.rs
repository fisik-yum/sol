use std::fmt::{Display, write};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token<'d> {
    Inspect,
    Load,
    Value(&'d str),
}

pub fn get_cmds_from_str(s: &str) -> Vec<Command<'_>> {
    let tokens = lex(s);
    parse(tokens)
}

fn lex<'l>(s: &'l str) -> Vec<Token<'l>> {
    let iter = s.trim_start().trim_end().split_whitespace();
    let mut result: Vec<Token> = vec![];
    for token_text in iter {
        result.push(match_token(token_text));
    }
    result
}
fn match_token<'s>(s: &'s str) -> Token<'s> {
    match s {
        "load" => Token::Load,
        "inspect" => Token::Inspect,
        _ => Token::Value(s),
    }
}

pub enum Command<'c> {
    Inspect(&'c str),
    Load(&'c str),
}
impl Display for Command<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inspect(s) => write!(f, "inspect {}", s),
            Self::Load(s) => write!(f, "load {}", s),
        }
    }
}
fn parse<'t>(tokens: Vec<Token<'t>>) -> Vec<Command<'t>> {
    let mut iter = tokens.iter().peekable();
    let mut commands: Vec<Command<'t>> = vec![];
    while let Some(token) = iter.peek() {
        match token {
            Token::Inspect => {
                iter.next();
                match iter.next() {
                    Some(Token::Value(s)) => {
                        commands.push(Command::Inspect(s));
                    }
                    _ => {
                        eprintln!("unexpected token: non-value")
                    }
                }
            }
            Token::Load => {
                iter.next();
                match iter.next() {
                    Some(Token::Value(s)) => {
                        commands.push(Command::Load(s));
                    }
                    _ => {
                        eprintln!("unexpected token: non-value")
                    }
                }
            }
            _ => {}
        }
    }
    commands
}
