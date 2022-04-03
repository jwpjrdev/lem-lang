use std::collections::HashMap;
use crate::ast::*;

pub fn interpret(file: File) {
    let mut var_store: HashMap<String, String> = HashMap::new();

    for node in file.nodes {
        match node {
            Node::Call(call) => {
                match call.ident.as_str() {
                    "println" => {
                        match call.args {
                            Args::String { value } => {
                                println!("{value}");
                            },
                            Args::Variable { ident } => {
                                let value = match var_store.get(&ident) {
                                    Some(value) => value,
                                    None => panic!("the variable {ident} does not exist"),
                                };
                                println!("{value}");
                            },
                        }
                    },
                    "print" => {
                        match call.args {
                            Args::String { value } => {
                                print!("{value}");
                            },
                            Args::Variable { ident } => {
                                let value = match var_store.get(&ident) {
                                    Some(value) => value,
                                    None => panic!("the variable {ident} does not exist"),
                                };
                                print!("{value}");
                            },
                        }
                    },
                    _ => {
                        eprintln!("unknown identifier: {}", call.ident);
                    },
                }
            },
            Node::VarDecl(var_decl) => {
                var_store.insert(var_decl.ident, var_decl.data);
            },
            Node::EOI => {
                break;
            },
        }
    }
}
