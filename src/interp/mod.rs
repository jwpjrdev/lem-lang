use std::collections::HashMap;
use std::cell::Cell;
use thiserror::Error;

use crate::ast::*;
pub use crate::interp::environment::*;

pub mod environment;

#[derive(Error, Debug)]
enum InterpreterError {
    #[error("")]
    DefineError,
    #[error("")]
    RedefineError,
}

#[derive(Debug)]
pub enum Command {
    Call { ident: String, args: Args },
    VarDecl { ident: String, value: Value },
    VarReassign { ident: String, value: Value },
    EnterBlock,
    ExitBlock,
    EOI,
}

pub struct Interpreter {
    pub environments: HashMap<usize, Environment>,
    pub environment: Environment,
    // TODO: environment is out of sync with the same one stored in environments. Perhaps only store the current environment in the environment variable, and not in environments as well? That way you can update any for loops through all environments (or lookups) to first try the current environment and then the environments map
    // TODO: write an executor/program that lets you step by step see the execution of the program so you can see exactly how state changes
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environments = HashMap::new();
        let environment = Environment::new(None);
        environments.insert(environment.id, environment.clone());

        Interpreter {
            environments: environments,
            environment: environment,
        }
    }

    pub fn new_env(&mut self) {
        let new_env = Environment::new(Some(self.environment.id));
        self.environments.insert(new_env.id, new_env.clone());
        self.environment = new_env;
    }

    pub fn destroy_env(&mut self) {
        if self.environment.parent.is_some() {
            let parent = self.environments.get(&self.environment.parent.unwrap()).unwrap();
            let old_id = self.environment.id.clone();
            self.environment = parent.clone();
            // self.environments.remove(&old_id);
        } else {
            // runtime error
            // println!("destroy_env: reached parent");
        }
    }

    pub fn fetch(&self, ident: &str) -> Option<&Value> {
        if self.environment.var_exists(ident) {
            return self.environment.get_var(ident);
        } else {
            let mut current_env = &self.environment;
            loop {
                if current_env.parent.is_some() {
                    current_env = self.environments.get(&current_env.parent.unwrap()).unwrap();
                    if current_env.var_exists(ident) {
                        return current_env.get_var(ident);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::Call { ident, args } => {
                match ident.as_str() {
                    "debug" => {
                        println!("{args:?}");
                    },
                    // todo: implement seperately & properly
                    "print" | "println" => {
                        if args.len() == 1 {
                            let value = args.get().first().unwrap();
                            match value {
                                Value::Ident(inner_ident) => {
                                    if self.environment.var_exists(inner_ident) {
                                        let inner_string = self.fetch(inner_ident).unwrap();
                                        println!("{}", inner_string);
                                    } else {
                                        // error
                                    }
                                },
                                Value::String(inner_string) => {
                                    print!("{}", inner_string);
                                },
                                _ => unimplemented!(),
                            }
                        } else {
                            // error
                        }
                    },
                    _ => unimplemented!(),
                }
            },
            Command::VarDecl { ident, value } => {
                match &value {
                    // let example2 = example;
                    // example is ident
                    // example2 is ref_ident
                    Value::Ident(ref_ident) => {
                        if self.environment.var_exists(ref_ident.as_str()) {
                            // let ref_value = self.environment.values.get(ref_ident.as_str()).unwrap().to_owned();
                            let ref_value = self.fetch(ref_ident).unwrap();
                            self.environment.define(ident, ref_value.clone());
                            // self.environment.values.insert(ident.clone(), ref_value);
                        } else {
                            // error
                        }
                    },
                    _ => {
                        println!("before: {:?}", self.environment.values);
                        let mut env = self.environment.clone();
                        env.define(ident, value);
                        self.environments.remove(&self.environment.id);
                        self.environments.insert(env.id, env);
                        println!("after: {:?}", self.environment.values);
                        // println!("{:?}", self.environment.values);
                        // self.environment.values.insert(ident, value);
                    }
                }
            },
            Command::VarReassign { ident, value } => {
                match &value {
                    // let example2 = example;
                    // example is ident
                    // example2 is refIdent
                    Value::Ident(ref_ident) => {
                        if self.environment.var_exists(ref_ident.as_str()) {
                            let ref_value = self.fetch(ref_ident).unwrap();

                            self.environment.define(ident.clone(), ref_value.clone());
                        } else {
                            // error
                        }
                    },
                    _ => {
                        println!("before: {:?}", self.environment.values);
                        let mut env = self.environment.clone();
                        env.define(ident, value);
                        self.environments.remove(&self.environment.id);
                        self.environments.insert(env.id, env);
                        println!("after: {:?}", self.environment.values);
                    }
                }
            },
            Command::EnterBlock => {
                println!("{:#?}", self.environments);
                self.new_env();
            }, // create new env
            Command::ExitBlock => {
                self.destroy_env();
            }, // destroy env
            Command::EOI => {
                // println!("{:#?}", self.environments);
            },
        }
    }
}






// impl Interpreter {
//     pub fn new(tree: File) -> Self {
//         Self { tree: tree }
//     }

//     pub fn interpret(&mut self) {
//         let mut nodes = self.tree.nodes.borrow_mut();
//         while !nodes.is_empty() {
//             let instr = nodes.pop_front().unwrap();
//             match instr {
//                 Node::Call { ident, args } => {
//                     println!("- call {:?} with {:?}", ident, args);
//                     if ident == String::from("print") {
//                         println!("{args:#?}");
//                     }
//                 }
//                 Node::VarDecl { ident, value } => {
//                     println!("- var decl {:?} as {:?}", ident, value);
//                 }
//                 Node::VarReassign { ident, value } => {
//                     println!("- var reassign {:?} as {:?}", ident, value);
//                 }

//                 Node::EOI => {} // terminate
//             };
//         }
//     }
// }

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
