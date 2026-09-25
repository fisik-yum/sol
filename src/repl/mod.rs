use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Config, Editor, Result};
use sol::sys::interpreter::{Environment, Pipeline};
use sol::sys::parser::parse;
use sol::sys::{tokenize, transforms};

pub struct REPL<'e> {
    editor: rustyline::Editor<(), MemHistory>,
    environment: Environment<'e>,
}

impl<'e> REPL<'e> {
    pub fn new() -> Self {
        let conf = Config::builder();
        let editor = Editor::with_history(conf.build(), MemHistory::new()).unwrap();
        let mut pipeline = Pipeline::new();
        pipeline.add_stage(&transforms::InteractiveMode);
        Self {
            editor,
            environment: Environment::new(pipeline),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let readline = self.editor.readline("#>>>  ");
            match readline {
                Ok(read) => {
                    let _ = self.editor.add_history_entry(read.as_str());
                    let leaked_str: &'static str = Box::leak(read.into_boxed_str());
                    let tokens = tokenize::Tokenizer::from(leaked_str);
                    match parse(tokens) {
                        Ok(ast) => {
                            match self.environment.interpret(ast) {
                                Ok(result) => println!("{}", result),
                                Err(e) => println!("Error: {:?}", e),
                            }
                        }
                        Err(e) => println!("Parse error: {:?}", e),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }
}
