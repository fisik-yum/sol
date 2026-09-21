use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Config, DefaultEditor, Editor, Result};
use sol::sys::ast::ASTNode;
use sol::sys::{self, Pipeline, Program, transforms};
mod interpreter;
struct Environment<'e> {
    editor: rustyline::Editor<(), MemHistory>,
    file_list: HashMap<&'e str, Program<'e>>,
    pipeline: Pipeline<'e>,
    program: Program<'e>,
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
            program: Program::default(),
        }
        // TODO: curr set up so that tal is not considered,
    }

    pub fn run(&self) -> Result<()> {
        // `()` can be used when no completer is required
        let mut rl = DefaultEditor::new()?;
        loop {
            let readline = rl.readline("#>>>  ");
            match readline {
                Ok(line) => {
                    let commands = self.pipeline.ingest(line.as_str()).unwrap();
                    for cmd in commands.root.get_children() {
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

    fn execute_commands(&self, p: Program) {
        for node in p.root.get_children().iter().cloned() {
            self.execute_instruction(node);
        }
    }

    fn execute_instruction(&self, n: ASTNode) {
        match n {
            ASTNode::Sequence(s,_)=>{
                let loc = self.program.root.insert_node(n);
                // i fogot what the args do
                self.program.symbols.insert(s, loc, loc)
            }
        }
    }
}
