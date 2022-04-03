// use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/lem.pest"]
pub struct LemParser;
