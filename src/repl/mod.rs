mod parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
pub fn run() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("#sol#  ");
        match readline {
            Ok(line) => {
                let cmds = parser::get_cmds_from_str(line.as_str());
                for cmd in cmds {
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
