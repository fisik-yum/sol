use std::{fs,path::Path};
mod tokenize;
mod parse;
mod ast;
fn main() {
    let p = Path::new("hello.sol");
    let f = fs::read(p);
    let t = String::from_utf8(f.unwrap()).unwrap();
    let tokenizer = tokenize::Tokenizer::from(t);
    for tok in tokenizer{
        println!("{}",tok)
    }
}
