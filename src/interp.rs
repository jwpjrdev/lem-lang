use std::collections::HashMap;
use crate::ast::*;

pub struct Builtin {
    pub ident: String,
    pub execute: Box<dyn Fn(Args)>,
}

pub struct Interpreter {
    pub builtins: HashMap<String, Builtin>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { builtins: HashMap::new() }
    }

    pub fn add_builtin(mut self, builtin: Builtin) -> Self {
        let identifier = builtin.ident.clone();
        self.builtins.insert(identifier.to_string(), builtin);
        self
    }

    pub fn interpret(self, file: File) -> Self {
        let mut var_store: HashMap<String, Box<Value>> = HashMap::new();

        for node in file.nodes {
            match node {
                Node::Call(call) => {
                    let builtin_ident = call.ident.as_str();

                    let builtin = match self.builtins.get(builtin_ident) {
                        Some(builtin) => builtin,
                        None => panic!("builtin '{builtin_ident} not found'"),
                    };

                    let mut flattened_args: Vec<Value> = vec![];

                    for arg in call.args.into_iter() {
                        match arg {
                            Value::String { value } => {
                                flattened_args.push(arg);
                            },
                            Value::VarRef { ident } => {
                                match var_store.get(&ident) {
                                    Some(value) => {
                                        flattened_args.push(value);
                                    }, 
                                    None => panic!("the variable {ident} was not found in scope"),
                                }
                            },
                            Value::TempNone => unreachable!(), // todo: get rid of TempNone
                        }
                    }

                    let args = Args::from_values(flattened_args);
                    (builtin.execute)(args);
                },
                Node::VarDecl(var_decl) => {
                    var_store.insert(var_decl.ident, Box::new(var_decl.data));
                },
                Node::EOI => {
                    break;
                },
            }
        }
        self
    }
}