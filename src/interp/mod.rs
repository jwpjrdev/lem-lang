use std::collections::LinkedList;

use crate::{ast::*, parser::Rule};
use pest::iterators::Pairs;

pub struct Interpreter {
    // pub instrs: LinkedList<Node>,
    pub tree: File,
}

impl Interpreter {
    pub fn new(tree: File) -> Self {
        Self { tree: tree }
    }

    pub fn interpret(&mut self) {
        let mut nodes = self.tree.nodes.borrow_mut();
        while !nodes.is_empty() {
            let instr = nodes.pop_front().unwrap();
            match instr {
                Node::Call { ident: ident, args } => {
                    println!("- call {:?} with {:?}", ident, args);
                    if ident == String::from("print") {
                        println!("{args:#?}");
                    }
                },
                Node::EOI => {}, // terminate
            };
        }
    }
}

// fn execute_builtin(&self, var_store: &HashMap<String, Value>, builtin_ident: String, args: Args) {
//     match builtin_ident.as_str() {
//         "println" => {
//             use std::io::{self, Write};
//             let mut out = io::sink(); // don't print for now

//             let count = args.count();
//             match count {
//                 0 => println!(),
//                 1 => {
//                     let value = args.into_iter().next().unwrap();
//                     match value {
//                         Value::String { value } => {
//                             out.write_all(b"{value}\n");
//                         },
//                         Value::VarRef { ident: _ } => {
//                             match self.unravel_var_ident(var_store, value) {
//                                 Value::String { value } => {
//                                     out.write_all(b"{value}\n");
//                                 },
//                                 _ => unreachable!(),
//                             }
//                         },
//                         _ => unreachable!(),
//                     }
//                 },
//                 _ => {
//                     panic!("builtin 'println' expected 1 argument, found {count}");
//                 },
//             }
//         },
//         "print" => {},
//         "debug" => {
//             println!("debug");
//             for arg in args.into_iter() {
//                 match arg {
//                     Value::String { value } => {
//                         println!("- value: {value}");
//                     },
//                     Value::VarRef { ident: _ } => unreachable!(),
//                     _ => {}, // todo: eliminate Value::TempNone
//                 }
//             }
//         },
//         _ => panic!("builtin '{builtin_ident}' not found"),
//     }
// }
