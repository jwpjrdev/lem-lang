use std::{cell::RefCell, collections::LinkedList, rc::Rc};
use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};

#[derive(Debug)]
pub struct File {
    pub nodes: Rc<RefCell<LinkedList<Node>>>,
}

#[derive(Debug)]
pub enum Node {
    Call { ident: String, args: Args },
    EOI,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Ident(String),
}

#[derive(Debug)]
pub struct Args {
    inner: Vec<Value>,
}

impl Args {
    fn new() -> Args {
        Args { inner: vec![] }
    }

    pub fn from_values(values: Vec<Value>) -> Args {
        Args { inner: values }
    }

    pub fn from_value(value: Value) -> Args {
        Args { inner: vec![value] }
    }

    pub fn has_args(self) -> bool {
        !self.inner.is_empty()
    }
}

impl IntoIterator for Args {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug)]
pub struct AST<'a> {
    nodes: Rc<RefCell<LinkedList<Node>>>,
    parse_tree: Pairs<'a, Rule>,
    next: Pair<'a, Rule>,
}

impl<'a> AST<'a> {
    pub fn new(tree: Pairs<'a, Rule>) -> Self {
        // scripts can't be empty—ensured in src/bin/lem.rs
        let next = tree.peek().clone().expect("an empty tree was provided");
        Self { nodes: Rc::new(RefCell::default()), parse_tree: tree, next: next }
    }

    pub fn parse_tree(&mut self) -> File {
        self.visit_rule();

        File {
            nodes: Rc::clone(&self.nodes),
        }
    }

    fn visit_rule(&mut self) {
        if let Some(next) = self.parse_tree.next() {
            match next.as_rule() {
                Rule::file => {
                    self.visit_file();
                },
                _ => unreachable!(),
            }
        }
    }
    
    fn visit_file(&mut self) {
        // println!("{:#?}", self.next);
        for next in self.next.clone().into_inner().into_iter() {
            match next.as_rule() {
                Rule::decl => {
                    self.next = next;
                    self.visit_decl();
                },
                Rule::EOI => {
                    self.visit_eoi();
                },
                _ => {},
            }
        }
    }

    fn visit_decl(&mut self) {
        for next in self.next.clone().into_inner().into_iter() {
            match next.as_rule() {
                Rule::call => {
                    self.next = next;
                    self.visit_call();
                },
                _ => {},
            }
        }
    }

    fn visit_call(&mut self) {
        let mut ident = String::new();
        let mut args = Args::new();

        for next in self.next.clone().into_inner() {
            match next.as_rule() {
                Rule::ident => {
                    self.next = next;
                    ident = self.visit_ident();
                },
                Rule::args => {
                    self.next = next;
                    args = self.visit_args();
                },
                _ => unreachable!(),
            }
        }

        self.nodes.borrow_mut().push_back(
            Node::Call {
                ident: ident,
                args: args,
            }
        );
    }

    fn visit_ident(&mut self) -> String {
        self.next.as_span().as_str().into()
    }

    fn visit_args(&mut self) -> Args {
        let mut values: Vec<Value> = vec![];
    
        for next in self.next.clone().into_inner() {
            match next.as_rule() {
                Rule::value => {
                    self.next = next;
                    values.push(self.visit_value());
                },
                _ => unreachable!(),
            }
        }
        
        Args::from_values(values)
    }

    fn visit_value(&mut self) -> Value {
        let next = self.next.clone().into_inner().next().unwrap();
        match next.as_rule() {
            Rule::string => {
                self.next = next;
                Value::String(self.visit_string())
            },
            _ => unimplemented!(),
        }
    }

    // This is a bit hacky but I couldn't quite get the grammar to
    // include beginning and trailing whitespace in raw strings.
    // #replace (instead of removing from beginning and end) only works
    // because "\"" already isn't allowed in strings.
    fn visit_string(&mut self) -> String {
        self.next.clone().as_span().as_str().to_string().replace("\"", "")
    }



    fn visit_eoi(&mut self) {
        self.nodes.borrow_mut().push_back(Node::EOI);
    }
}
