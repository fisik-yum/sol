use clap::Parser;
use sol::sys::{self, Pipeline, transforms};
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

    // print mathrai count
    #[arg(
        short = 'm',
        long,
        default_value_t = false,
        help = "print cumulative mathrai count"
    )]
    mat: bool,
    // print akshara count
    #[arg(
        short = 'a',
        long,
        default_value_t = false,
        help = "print cumulative + relative akshara count"
    )]
    aksh: bool,
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
    let sym = sys::SymbolTable::new(&tree).unwrap();

    let mut pipe = Pipeline::new();
    pipe.add_stage(&transforms::RemoveInteractive);
    let _env = sys::Environment::new(pipe);

    // ARG HANDLING CODE
    if args.tree {
        let _ = tree.prettyprint();
    }

    if args.mat {
        let size = match sys::stdlib::mat::count_m(&tree, &sym) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e.report(&loc, &t));
                std::process::exit(1);
            }
        };
        println!("Mathrai: {}", size);
    }

    if args.aksh {
        let size = match sys::stdlib::aks::count_a(&tree, &sym) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e.report(&loc, &t));
                std::process::exit(1);
            }
        };
        print!("PartialAks: {}", size);
        // bad code ngl: we gotta fix this
        //let cycles = talm::ava::Avartana::from_standard(size, prog.cycle);
        //println!("({})", cycles);
    }

    let duration = (time::Instant::now() - start).as_micros();
    println!("finished executing {loc} in {duration} microseconds");
}
