use crate::callable::CallableRet;
use crate::{interpreter::RuntimeException, token::*};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub map: HashMap<String, CallableRet>,
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
    pub fn define(&mut self, name: &str, value: Option<CallableRet>) {
        self.map.insert(
            String::from(name),
            value.unwrap_or(CallableRet::Value(LiteralValue::NilLiteral)),
        );
    }
    pub fn assign(
        &mut self,
        name: &Token,
        value: CallableRet,
    ) -> Result<CallableRet, RuntimeException> {
        match self.map.entry(name.lexeme.clone()) {
            Entry::Occupied(mut occupied) => Ok(occupied.insert(value)),
            Entry::Vacant(_) => match &mut self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(RuntimeException::UndefinedVar(name.clone())),
            },
        }
    }
    pub fn get(&self, name: &Token) -> Result<CallableRet, RuntimeException> {
        match self.map.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(RuntimeException::UndefinedVar(name.clone())),
            },
        }
    }
}
