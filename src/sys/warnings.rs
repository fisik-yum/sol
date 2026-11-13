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
                write!(f, "Environment => {}", s)
            }
            Self::ParseError(s) => {
                let (row, col) = s.to_line();
                let segment = s.to_line_segment(row);
                let _ = writeln!(f, "Error parsing on line {}:{}", row, col);
                let _ = writeln!(f, "{}", segment.trim_end());
                write!(f, "{}^ Error encountered here", " ".repeat(col - 1))
            }
            Self::GlobalError(s) => {
                write!(f, "Global => {}", s)
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
}
