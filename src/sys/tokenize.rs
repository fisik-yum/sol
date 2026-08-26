use ariadne::{Color, Label, Report, ReportKind, sources};
use chumsky::{input::ValueInput, prelude::*};
use std::{collections::HashMap, env, fmt, fs};

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
enum Token<'src> {
    Sol,
    Figure(usize),
    Ctrl(char),
    Ident(&'src str),
    Seq,
    Tal,
    Nad,
    Mat,
    Aks,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Sol => write!(f, "sol"),
            Token::Seq => write!(f, "seq"),
            Token::Tal => write!(f, "tal"),
            Token::Nad => write!(f, "nad"),
            Token::Figure(u) => write!(f, "{u}"),
            Token::Ctrl(c) => write!(f, "{c}"),
            Token::Ident(s) => write!(f, "{s}"),
            Token::Mat => write!(f, "mat"),
            Token::Aks => write!(f, "sol"),
        }
    }
}

fn lexer<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, Span>>> {
    let fig = text::int(10)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Figure);

    let ctrl = one_of("(){}").map(Token::Ctrl);

    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "seq" => Token::Seq,
        "aks" => Token::Aks,
        "mat" => Token::Mat,
        "sol" => Token::Sol,
        _ => Token::Ident(ident),
    });

    let token = fig.or(ctrl).or(ident);
    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

pub enum Value{}
