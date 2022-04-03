use std::collections::HashMap;
use crate::ast::*;

#[derive(Clone, Copy)]
pub struct Builtin {
    pub ident: &'static str,
    pub execute: &'static dyn Fn(String),
}

pub struct Interpreter {
    pub builtins: HashMap<String, Builtin>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            builtins: HashMap::new(),
        }
    }

    pub fn add_builtin(mut self, builtin: &Builtin) -> Self {
        let identifier = builtin.ident;
        self.builtins.insert(identifier.to_string(), *builtin);
        self
    }

    pub fn interpret(self, file: File) -> Self {
        let mut var_store: HashMap<String, String> = HashMap::new();

        for node in file.nodes {
            match node {
                Node::Call(call) => {
                    let builtin_ident = call.ident.as_str();

                    let to_run = match self.builtins.get(builtin_ident) {
                        Some(builtin) => builtin,
                        None => panic!("builtin '{builtin_ident} not found'"),
                    };
                    let args = match call.args {
                        Args::String { value } => {
                            value
                        },
                        Args::Variable { ident } => {
                            match var_store.get(&ident) {
                                Some(value) => value.to_string(),
                                None => panic!("the variable {ident} does not exist"),
                            }
                        },
                    };
                    (to_run.execute)(args);
                    // match call.ident.as_str() {
                    //     "println" => {
                    //         match call.args {
                    //             Args::String { value } => {
                    //                 println!("{value}");
                    //             },
                    //             Args::Variable { ident } => {
                    //                 let value = match var_store.get(&ident) {
                    //                     Some(value) => value,
                    //                     None => panic!("the variable {ident} does not exist"),
                    //                 };
                    //                 println!("{value}");
                    //             },
                    //         }
                    //     },
                    //     "print" => {
                    //         match call.args {
                    //             Args::String { value } => {
                    //                 print!("{value}");
                    //             },
                    //             Args::Variable { ident } => {
                    //                 let value = match var_store.get(&ident) {
                    //                     Some(value) => value,
                    //                     None => panic!("the variable {ident} does not exist"),
                    //                 };
                    //                 print!("{value}");
                    //             },
                    //         }
                    //     },
                    //     _ => {
                    //         eprintln!("unknown identifier: {}", call.ident);
                    //     },
                    // }
                },
                Node::VarDecl(var_decl) => {
                    var_store.insert(var_decl.ident, var_decl.data);
                },
                Node::EOI => {
                    break;
                },
            }
        }
        self
    }
}