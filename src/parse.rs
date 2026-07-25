use crate::tokenize::Tokenizer;

// INFO: we will figure this nonsense out
// after I teach my self state machines
enum ParserState {
    FnLookup,
    BlockOpen,
    Wait,
}

pub struct Parser {
    tok_stream: Tokenizer,
    state: ParserState,
}

impl From<Tokenizer> for Parser {
    fn from(value: Tokenizer) -> Self {
        Self {
            tok_stream: value,
            state: ParserState::Wait,
        }
    }
}
