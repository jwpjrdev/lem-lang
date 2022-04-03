use pest::iterators::{Pair, Pairs};
use crate::parser::Rule;

#[derive(Debug)]
pub struct File {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    Call(Call),
    EOI,
}

#[derive(Debug)]
pub struct Call {
    pub ident: String,
    pub args: String,
}

// - file
//   - call
//     - ident: "println"
//     - args > string > raw_string: "Hello, world!"
//   - EOI: ""

// todo: move visitors into struct that holds parse_tree
pub fn generate_ast(mut parse_tree: Pairs<Rule>) -> File {
    let mut nodes = vec![];

    if let Some(file) = parse_tree.next() {
        for inner in file.into_inner().into_iter() {
            match inner.as_rule() {
                Rule::call => {
                    visit_call(inner, &mut nodes);
                },
                Rule::EOI => {
                    visit_eoi(&mut nodes);
                },
                _ => {},
            }
        }
    }
    
    File { nodes: nodes }
}

fn visit_call(call: Pair<Rule>, nodes: &mut Vec<Node>) {
    let mut ident = "";
    let mut args = "";
    for inner in call.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                ident = inner.as_span().as_str();
            },
            Rule::args => {
                let string = inner.into_inner().next().unwrap();
                let raw_string = string.into_inner().next().unwrap();
                match raw_string.as_rule() {
                    Rule::raw_string => {
                        args = raw_string.as_span().as_str();
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    nodes.push(
        Node::Call(Call {
            ident: ident.into(),
            args: args.into(),
        })
    );
}

fn visit_eoi(nodes: &mut Vec<Node>) {
    nodes.push(Node::EOI);
}
