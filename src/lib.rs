use pest::Parser;
use crate::parser::{LemParser, Rule};
// use crate::ast::File;

pub mod ast;
pub mod interp;
pub mod parser;

// todo: return exit code
pub fn execute_script(script: String) {
    let parse_tree = LemParser::parse(Rule::file, &script)
        .unwrap_or_else(|err| panic!("{err}"));

    // println!("{parse_tree:#?}");

    let syntax_tree = crate::ast::generate_ast(parse_tree);
    
    println!("{syntax_tree:#?}");
    crate::interp::interpret(syntax_tree);
}
