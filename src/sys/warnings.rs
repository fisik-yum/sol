pub struct Error {
    pos: Option<usize>,
    msg: String,
}

impl Error {
    pub fn at(pos: usize, msg: impl Into<String>) -> Self {
        Self {
            pos: Some(pos),
            msg: msg.into(),
        }
    }

    pub fn eof(msg: impl Into<String>) -> Self {
        Self {
            pos: None,
            msg: format!("{} (unexpected end of input)", msg.into()),
        }
    }

    pub fn global(msg: impl Into<String>) -> Self {
        Self {
            pos: None,
            msg: msg.into(),
        }
    }

    pub fn report(&self, filename: &str, src: &str) -> String {
        match self.pos {
            Some(pos) => {
                let (line, col) = Self::line_col(src, pos);
                format!("{filename}:{line}:{col}: error: {}", self.msg)
            }
            None => format!("{filename}: error: {}", self.msg),
        }
    }

    fn line_col(src: &str, pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (i, c) in src.char_indices() {
            if i >= pos {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
}
