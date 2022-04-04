use pest::iterators::{Pair, Pairs};
use crate::parser::Rule;

#[derive(Debug)]
pub struct File {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    Call(Call),
    VarDecl(VarDecl),
    EOI,
}

#[derive(Debug)]
pub struct Call {
    pub ident: String,
    pub args: Args,
}

#[derive(Debug)]
pub enum Args {
    String { value: String },
    Variable { ident: String },
    None,
}

#[derive(Debug)]
pub struct VarDecl {
    pub ident: String,
    pub data: String,
}

// todo: move visitors into struct that holds parse_tree
pub fn generate_ast(mut parse_tree: Pairs<Rule>) -> File {
    let mut nodes = vec![];

    if let Some(file) = parse_tree.next() {
        for inner in file.into_inner().into_iter() {
            match inner.as_rule() {
                Rule::decl => {
                    visit_decl(inner, &mut nodes);
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

fn visit_decl(decl: Pair<Rule>, nodes: &mut Vec<Node>) {
    let inner = decl.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::call => {
            visit_call(inner, nodes);
        },
        Rule::var_decl => {
            visit_var_decl(inner, nodes);
        },
        _ => {},
    }
}

fn visit_call(call: Pair<Rule>, nodes: &mut Vec<Node>) {
    let mut ident = String::new();
    let mut args = Args::None;

    for inner in call.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                ident = visit_ident(inner);
            },
            Rule::args => {
                args = visit_args(inner);
            },
            _ => unreachable!(),
        }
    }

    nodes.push(
        Node::Call(Call {
            ident: ident,
            args: args,
        })
    );
}

fn visit_var_decl(var_decl: Pair<Rule>, nodes: &mut Vec<Node>) {
    let mut ident = String::new();
    let mut data = String::new();

    for inner in var_decl.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                ident = visit_ident(inner);
            },
            Rule::string => {
                data = visit_string(inner);
            },
            _ => unreachable!(),
        }
    }
    
    nodes.push(
        Node::VarDecl(VarDecl {
            ident: ident,
            data: data,
        })
    );
}

fn visit_args(args: Pair<Rule>) -> Args {
    let inner = args.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::string => {
            Args::String {
                value: visit_string(inner),
            }
        },
        Rule::ident => {
            Args::Variable {
                ident: visit_string(inner),
            }
        }
        _ => unreachable!(),
    }
}

// This is a bit hacky but I couldn't quite get the grammar to
// include beginning and trailing whitespace in raw strings.
// #replace (instead of removing from beginning and end) only works
// because "\"" already isn't allowed in strings.
fn visit_string(string: Pair<Rule>) -> String {
    string.as_span().as_str().to_string().replace("\"", "")
}

fn visit_ident(ident: Pair<Rule>) -> String {
    ident.as_span().as_str().into()
}

fn visit_eoi(nodes: &mut Vec<Node>) {
    nodes.push(Node::EOI);
}
