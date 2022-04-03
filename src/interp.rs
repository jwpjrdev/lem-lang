use crate::ast::{File, Node};

pub fn interpret(file: File) {
    'loop2: for node in file.nodes {
        match node {
            Node::Call(call) => {
                match call.ident.as_str() {
                    "println" => {
                        println!("{}", call.args);
                    },
                    "print" => {
                        print!("{}", call.args);
                    },
                    _ => {
                        eprintln!("unknown identifier: {}", call.ident);
                    },
                }
            },
            Node::EOI => {
                println!("[lem] finished");
                break 'loop2;
            },
        }
    }
}