use std::time;
use std::{collections::VecDeque, fs, path::Path};
mod ast;
mod calc;
mod parser;
mod tokenize;
mod transform;
mod warnings;
fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front();

    let start = time::Instant::now();
    let loc = args.pop_front().unwrap();
    let p = Path::new(loc.as_str());
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

    let duration = (time::Instant::now() - start).as_micros();
    println!("finished parsing {loc} in {duration} microseconds");
    tree.prettyprint();

    let size = calc::mat::count_m(&tree, &tree, &table);
    println!("tree size: {}", size);

    let size = calc::aks::count_a(&tree, &table);
    println!("tree size: {}", size);
}
