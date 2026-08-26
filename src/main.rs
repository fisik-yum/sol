use clap::Parser;
use sol::sys::{self, parser, tokenize};
use std::time;
use std::{fs, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long = "file")]
    f: String,

    // print tree
    #[arg(long, default_value_t = false)]
    tree: bool,

    // print akshara count
    #[arg(long, default_value_t = false)]
    aksh: bool,
}
fn main() {
    let args = Args::parse();

    let loc = &args.f.as_str();
    let p = Path::new(&loc);

    let start = time::Instant::now();

    let f = fs::read(p);
    let t = String::from_utf8(f.unwrap()).unwrap();

    let tokenizer = tokenize::Tokenizer::from(t.as_str());

    let (tree, table) = match parser::parse(tokenizer) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.report(&loc, &t));
            std::process::exit(1);
        }
    };

    if args.tree {
        //tree.prettyprint();
        println!("tree printing disabled")
    }

    /*
    let size = match sol::calc::mat::count_m(&tree, &tree, &table) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.report(&loc, &t));
            std::process::exit(1);
        }
    };
    println!("parse size: {}", size);

    if args.aksh {
        let size = match sol::calc::aks::count_a(&tree, &table) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e.report(&loc, &t));
                std::process::exit(1);
            }
        };
        println!("tree size: {}", size);
    }
    */
    let _ = sys::execute::execute(&tree, &table);

    let duration = (time::Instant::now() - start).as_micros();
    println!("finished executing {loc} in {duration} microseconds");
}
