// extern crate pest;
#[macro_use]
extern crate pest_derive;

 pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "cha.pest"]
pub struct CSVParser;

fn main() {
    let unparsed_file = fs::read_to_string("src/number.csv").expect("cannot read file");
    let parse_result = CSVParser::parse(Rule::statement, &unparsed_file);

    match parse_result {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap().into_inner();
            for item in pair {
                println!("{:?}", item.as_str());
            }
        }
        Err(_err) => {}
    }
}
