use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Config, DefaultEditor, Editor, Result};
use sol::sys::{Pipeline, Program, transforms};
mod interpreter;
struct Environment<'e> {
    editor: rustyline::Editor<(), MemHistory>,
    file_list: HashMap<&'e str, Program<'e>>,
    pipeline: Pipeline<'e>,
}

impl<'e> Environment<'e> {
    pub fn new() -> Self {
        let conf = Config::builder();
        let editor = Editor::with_history(conf.build(), MemHistory::new()).unwrap();
        let mut pipeline = Pipeline::new();
        pipeline.add_stage(&transforms::InteractiveMode);
        Self {
            editor: editor,
            file_list: HashMap::new(),
            pipeline: pipeline,
        }
    }

    pub fn run(&self) -> Result<()> {
        // `()` can be used when no completer is required
        let mut rl = DefaultEditor::new()?;
        loop {
            let readline = rl.readline("#>>>  ");
            match readline {
                Ok(line) => {
                    let commands = self.pipeline.ingest(line.as_str()).unwrap();
                    for cmd in commands {
                        println!("{}", cmd)
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
