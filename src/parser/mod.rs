use pest::error::Error;
use pest::error::LineColLocation;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/lem.pest"]
pub struct LemParser;

impl LemParser {
    pub fn parse_with_timer(script: &String) -> Pairs<'_, Rule> {
        use pest::Parser;
        match LemParser::parse(Rule::file, script) {
            Ok(parse_tree) => parse_tree,
            Err(err) => {
                println!("position: {}", match err.line_col {
                    LineColLocation::Pos((line, col)) => format!("{}:{}", line, col),
                    LineColLocation::Span(line, col) => format!("{:?}:{:?}", line, col), // what's this for??
                });
                println!("location: {:?}", err.location);
                println!("variant: {:?}", err.variant);
                println!("line: {}", err.line());
                let new_err = err.renamed_rules(|rule| {
                    match *rule {
                        Rule::value => "string".to_owned(),
                        _ => "thing".to_owned(),
                    }
                });
                panic!("{new_err}");
            }
        }
            // .unwrap_or_else(|err| panic!("{err}"));
    }
}
