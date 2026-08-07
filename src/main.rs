use std::{fs, path::Path};
mod ast;
mod parse;
mod tokenize;
fn main() {
    let p = Path::new("hello.sol");
    let f = fs::read(p);
    let t = String::from_utf8(f.unwrap()).unwrap();
    let tokenizer = tokenize::Tokenizer::from(t.as_str());
    //for tok in tokenizer{
    //    println!("{}",tok)
    //}
    let mut parser = parse::Parser::from(tokenizer);
    let tree = parser.parse();
    println!("finished parse")
}
