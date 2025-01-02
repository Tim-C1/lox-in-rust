use crate::{interpreter::RuntimeError, token::*};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    map: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: None,
            map: HashMap::new(),
        }))
    }
    pub fn new_with_enclosing(enclosing: &Rc<RefCell<Self>>) -> Environment {
        Self {
            enclosing: Some(Rc::clone(enclosing)),
            map: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: &str, value: Option<LiteralValue>) {
        self.map.insert(
            String::from(name),
            value.unwrap_or(LiteralValue::NilLiteral),
        );
    }
    pub fn assign(
        &mut self,
        name: &Token,
        value: LiteralValue,
    ) -> Result<LiteralValue, RuntimeError> {
        match self.map.entry(name.lexeme.clone()) {
            Entry::Occupied(mut occupied) => Ok(occupied.insert(value)),
            Entry::Vacant(_) => match &mut self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(RuntimeError::UndefinedVar(name.clone())),
            },
        }
    }
    pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        match self.map.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(RuntimeError::UndefinedVar(name.clone())),
            },
        }
    }
}
