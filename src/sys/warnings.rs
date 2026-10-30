use std::fmt::Display;

use crate::sys::tokenize::Span;

#[derive(Debug)]
pub enum Error<'src> {
    ParseError(Span<'src>),
    EnvironmentError(String),
    GlobalError(String),
}
impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvironmentError(s) => {
                write!(f, "Environment: {}", s)
            }
            Self::ParseError(span) => {
                let _ = writeln!(f, "Parser:");
                write!(f, "{}", Self::write_span(*span))
            }
            Self::GlobalError(s) => {
                write!(f, "Global: {}", s)
            }
        }
    }
}
impl<'src> Error<'src> {
    pub fn parser(span: Span<'src>) -> Self {
        return Self::ParseError(span);
    }
    pub fn environment(msg: String) -> Self {
        return Self::EnvironmentError(msg);
    }
    pub fn global(msg: String) -> Self {
        return Self::GlobalError(msg);
    }
    fn write_span(span: Span) -> String {
        let mut sb = String::with_capacity(100);
        let (r, c) = span.to_line();
        sb.push_str(format!("({r},{c})").as_str());
        return sb;
    }
}
