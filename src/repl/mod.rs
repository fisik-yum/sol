use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Config, Editor, Result};
use sol::sys::interpreter::{Environment, ExecutionResult, Pipeline};
use sol::sys::parser::parse;
use sol::sys::tokenize;

pub struct REPL<'e> {
    editor: rustyline::Editor<(), MemHistory>,
    environment: Environment<'e>,
    pipeline: Pipeline<'e>,
    running: ExecutionResult,
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
            running: ExecutionResult::default(),
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
                                let (res, fields) = self.environment.execute(result.0).unwrap();
                                for f in fields {
                                    println!("{}", f)
                                }
                                println!("current: {}", &res);
                                self.running.add(res);
                                println!("global: {}", self.running);
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
