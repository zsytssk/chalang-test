// extern crate pest;
#[macro_use]
extern crate pest_derive;

 pest::Parser;
use std::fs;

use std::env;

#[derive(Parser)]
#[grammar = "cha.pest"]
pub struct CSVParser;

fn main() {
    let path = env::current_dir().expect("err");
    println!("{}", path.display());
    let unparsed_file = fs::read_to_string("src/number.csv").expect("cannot read file");
    let file = CSVParser::parse(Rule::enclosed, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    for enclosed in file.into_inner() {
        match enclosed.as_rule() {
            Rule::number => {
                println!("{}", enclosed.as_str())
            }
            _ => unreachable!(),
        }
    }
}
