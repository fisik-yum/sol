use clap::Parser;
use sol::sys::interpreter::Pipeline;
use sol::sys::{self, transforms};
// mod repl;
use std::time;
use std::{fs, path::Path};
#[derive(Parser, Debug)]
#[command(version, about="command-line solkattu verification program", long_about = None)]
struct Args {
    #[arg(short = 'f', default_value_t="".to_string() ,long = "file")]
    f: String,

    // open repl
    #[arg(short = 'r', long, default_value_t = false, help = "run repl")]
    repl: bool,

    // print tree
    #[arg(short = 't', long, default_value_t = false, help = "print parse tree")]
    tree: bool,
}
fn main() {
    let args = Args::parse();

    if args.repl {
        //repl::run();
    }

    let loc = &args.f.as_str();
    let p = Path::new(&loc);

    let start = time::Instant::now();

    let f = fs::read(p);
    let t = String::from_utf8(f.unwrap()).unwrap();

    let tokens = sys::tokenize::Tokenizer::from(t.as_str());

    let tree = sys::parser::parse(tokens).unwrap();

    let mut pipe = Pipeline::new();
    pipe.add_stage(&transforms::RemoveInteractive);
    let env = sys::interpreter::Environment::new(pipe);

    // ARG HANDLING CODE
    if args.tree {
        let _ = tree.prettyprint();
    }

    let res = env.interpret(tree).unwrap();
    println!("{}", res);
    let duration = (time::Instant::now() - start).as_micros();
    println!("finished executing {loc} in {duration} microseconds");
}
