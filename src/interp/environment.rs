use std::collections::HashMap;
use std::cell::Cell;

use crate::ast::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub id: usize,
    pub values: HashMap<String, Value>, // todo: remove pub
    pub parent: Option<usize>,
}

thread_local!(static ENVIRONMENT_ID: Cell<usize> = Cell::new(0));

impl Environment {
    // use Interpreter::new_env()
    pub fn new(parent: Option<usize>) -> Self {
        ENVIRONMENT_ID.with(|thread_id| {
            let id = thread_id.get();
            // if a program gets big enough, this could overflow. todo: find new id solution that won't run out. uuids?
            thread_id.set(id + 1);
            Environment { id: id, values: HashMap::new(), parent: parent }
        })
    } 

    pub fn define(&mut self, ident: String, value: Value) {
        println!("define in: {}", self.id);
        if !self.values.contains_key(ident.as_str()) {
            self.values.insert(ident, value);
        } else {
            // error
        }
        println!("{:?}", self.values);
    }

    pub fn redefine(&mut self, ident: String, value: Value) {
        if self.values.contains_key(ident.as_str()) {
            self.values.insert(ident, value);
        } else {
            // error
        }
    }

    pub fn var_exists(&self, ident: &str) -> bool {
        self.values.contains_key(ident)
    }

    pub fn get_var(&self, ident: &str) -> Option<&Value> {
        self.values.get(ident)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{Environment, Value};

    #[test]
    fn new() {
        let env = Environment::new(None);

        assert_eq!(env.parent, None);
        assert!(env.values.is_empty());
    }

    #[test]
    fn get_var() {
        let mut env = Environment::new(None);

        assert_eq!(env.values, HashMap::new());

        env.values.insert("test".to_string(), Value::String("test".to_string()));
        
        let mut map = HashMap::new();
        map.insert(String::from("test"), Value::String(String::from("test")));

        assert_eq!(env.values, map);
    }

    #[test]
    fn define_var() {
        let mut env = Environment::new(None);
        env.define(String::from("test"), Value::String(String::from("test")));

        let mut map = HashMap::new();
        map.insert(String::from("test"), Value::String(String::from("test")));

        assert_eq!(env.values, map);
    }
}
