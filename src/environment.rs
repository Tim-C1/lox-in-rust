use crate::{interpreter::RuntimeError, token::*};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Environment {
    map: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
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
            Entry::Vacant(_) => Err(RuntimeError::UndefinedVar(name.clone())),
        }
    }
    pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        match self.map.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => Err(RuntimeError::UndefinedVar(name.clone())),
        }
    }
}