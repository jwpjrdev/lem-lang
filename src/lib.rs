use pest::Parser;
use stopwatch::Stopwatch;

use crate::parser::{LemParser, Rule};
use crate::interp::Interpreter;

pub mod parser;
pub mod interp;

// todo: return exit code & execution history
pub fn execute_script(script: &String) {
    let sw = Stopwatch::start_new();
    let mut parse_sw = Stopwatch::start_new();
    let parse_tree = LemParser::parse_with_timer(&script);
    parse_sw.stop();

    println!("{parse_tree:#?}");

    // let mut ast_sw = Stopwatch::start_new();
    // let syntax_tree = SyntaxTree::new(parse_tree);
    // ast_sw.stop();

    // println!("{syntax_tree:#?}");

    // let mut interp_sw = Stopwatch::start_new();
    // let interpreter = Interpreter;
    // interpreter.interpret(syntax_tree);
    // interp_sw.stop();

    println!("\nFinished in {}μs", sw.elapsed().as_micros());
    println!("- parser: {}μs", parse_sw.elapsed().as_micros());
    // println!("- ast: {}μs", ast_sw.elapsed().as_micros());
    // println!("- interp: {}μs", interp_sw.elapsed().as_micros());
}
