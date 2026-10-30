use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Config, Editor, Result};
use sol::sys::interpreter::{Environment, Pipeline};
use sol::sys::parser::parse;
use sol::sys::tokenize;

pub struct REPL<'e> {
    editor: rustyline::Editor<(), MemHistory>,
    environment: Environment<'e>,
    pipeline: Pipeline<'e>,
}

impl<'e> REPL<'e> {
    pub fn new() -> Self {
        let conf = Config::builder();
        let editor = Editor::with_history(conf.build(), MemHistory::new()).unwrap();
        //pipeline.add_stage(&transforms::InteractiveMode);
        Self {
            editor,
            environment: Environment::new(),
            pipeline: Pipeline::new(),
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
                        Ok(ast) => match self.pipeline.to_bytecode(ast) {
                            Ok(result) => {
                                self.environment.ingest_memory(result.1);
                                let res = self.environment.execute(result.0).unwrap();
                                println!("{} | {}", res.0, res.1);
                            }
                            Err(e) => println!("Error: {:?}", e),
                        },
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
