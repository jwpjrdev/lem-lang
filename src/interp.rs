use std::collections::HashMap;
use crate::ast::*;

pub struct Builtin {
    pub ident: String,
    pub execute: Box<dyn Fn(Option<String>)>,
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

    pub fn add_builtin(mut self, builtin: Builtin) -> Self {
        let identifier = builtin.ident.clone();
        self.builtins.insert(identifier.to_string(), builtin);
        self
    }

    pub fn interpret(self, file: File) -> Self {
        let mut var_store: HashMap<String, String> = HashMap::new();

        for node in file.nodes {
            match node {
                Node::Call(call) => {
                    let builtin_ident = call.ident.as_str();

                    let builtin = match self.builtins.get(builtin_ident) {
                        Some(builtin) => builtin,
                        None => panic!("builtin '{builtin_ident} not found'"),
                    };
                    let args: Option<String> = match call.args {
                        Args::String { value } => {
                            Some(value)
                        },
                        Args::Variable { ident } => {
                            match var_store.get(&ident) {
                                Some(value) => Some(value.to_string()),
                                None => panic!("the variable {ident} does not exist"),
                            }
                        },
                        Args::None => {
                            None
                        },
                    };

                    (builtin.execute)(args);
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