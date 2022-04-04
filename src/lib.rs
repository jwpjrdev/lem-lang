use pest::Parser;

use crate::parser::{LemParser, Rule};
use crate::interp::{Builtin, Interpreter};

pub mod ast;
pub mod interp;
pub mod parser;

// todo: return exit code & execution history
pub fn execute_script(script: String) {
    let parse_tree = LemParser::parse(Rule::file, &script)
        .unwrap_or_else(|err| panic!("{err}"));

    // println!("{parse_tree:#?}");

    let syntax_tree = crate::ast::generate_ast(parse_tree);
    
    // println!("{syntax_tree:#?}");

    Interpreter::new()
        .add_builtin(Builtin {
            ident: "println".into(),
            execute: Box::new(|args| {
                match args {
                    Some(args) => {
                        println!("{args}");
                    },
                    None => {
                        println!();
                    },
                }
            }),
        })
        .add_builtin(Builtin {
            ident: "print".into(),
            execute: Box::new(|args| {
                match args {
                    Some(args) => {
                        print!("{args}");
                    },
                    None => {}
                }
            }),
        })
        .interpret(syntax_tree);
}
