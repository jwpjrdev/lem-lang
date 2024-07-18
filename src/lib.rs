use std::collections::LinkedList;

use pest::Parser;
use stopwatch::Stopwatch;

use crate::ast::{AST, Node};
use crate::interp::{Command, Interpreter};
use crate::parser::{LemParser, Rule};

pub mod ast;
pub mod interp;
pub mod parser;

// todo: return exit code & execution history
pub fn execute_script(script: &String) {
    let sw = Stopwatch::start_new();
    let mut parse_sw = Stopwatch::start_new();
    let tree = LemParser::parse_with_timer(&script);
    parse_sw.stop();

    // println!("{tree:#?}");
    let ast = AST::new(tree).parse_tree();
    // println!("{ast:#?}");
    
    let mut commands = Vec::new();

    let mut nodes = ast.nodes.borrow_mut();
    while !nodes.is_empty() {
        let instr = nodes.pop_front().unwrap();
        match instr {
            Node::Call { ident, args} => {
                commands.push(Command::Call { ident: ident, args: args });
            },
            Node::VarDecl { ident, value } => {
                commands.push(Command::VarDecl { ident: ident, value: value });
            },
            Node::VarReassign { ident, value } => {
                commands.push(Command::VarReassign { ident: ident, value: value });
            },
            Node::EnterBlock => commands.push(Command::EnterBlock),
            Node::ExitBlock => commands.push(Command::ExitBlock),
            Node::EOI => {
                commands.push(Command::EOI);
            },
        }
    }

    let mut interpreter = Interpreter::new();
    
    println!("{commands:#?}");
    for command in commands {
        interpreter.interpret(command);
    }


    //let mut nodes = self.tree.nodes.borrow_mut();
//         while !nodes.is_empty() {
//             let instr = nodes.pop_front().unwrap();
//             match instr {
//                 Node::Call { ident, args } => {
    
    // let interpreter = Interpreter::new();

    // let mut ast_sw = Stopwatch::start_new();
    // ast_sw.stop();
    
    // let mut interp_sw = Stopwatch::start_new();
    // interp_sw.stop();

    println!("\nFinished in {}μs", sw.elapsed().as_micros());
    println!("- parser: {}μs", parse_sw.elapsed().as_micros());
    // println!("- ast: {}μs", ast_sw.elapsed().as_micros());
    // println!("- interp: {}μs", interp_sw.elapsed().as_micros());
}
