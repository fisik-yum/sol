use std::time;
use std::{collections::VecDeque, fs, path::Path};
mod ast;
mod math;
mod parse;
mod tokenize;
mod solve;
fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front();

    let start = time::Instant::now();
    let loc = args.pop_front().unwrap();
    let p = Path::new(loc.as_str());
    let f = fs::read(p);

    let t = String::from_utf8(f.unwrap()).unwrap();

    let tokenizer = tokenize::Tokenizer::from(t.as_str());

    let parser = parse::Parser::from(tokenizer);
    let (tree, table) = match parser.parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.report(&loc, &t));
            std::process::exit(1);
        }
    };

    let duration = (time::Instant::now() - start).as_micros();
    println!("finished parsing {loc} in {duration} microseconds");
    tree.prettyprint();

    let size = math::count_m(&tree, &tree, &table);
    println!("tree size: {size}");
}
