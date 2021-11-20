use super::super::interpreter::*;
use super::super::scanner::{scanner::*, tokens::*};
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    LoxClass,
    LoxFunction,
    LoxInstance,
    LoxNativeFunction,
    LoxNativeClass,
    LiteralValue,
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, EnvironmentValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: EnvironmentValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<EnvironmentValue, Error> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        }
        if let Some(enclosing) = self.enclosing.as_mut() {
            return enclosing.get(name);
        }
        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }

    pub fn get_env_by_distance(&mut self, distance: usize) -> &mut Environment {
        let mut environment = self;
        let mut distance = distance;

        loop {
            if distance > 0 {
                if let Some(ref mut enclosing) = environment.enclosing {
                    environment = enclosing;
                    distance -= 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        environment
    }

    pub fn assign(&mut self, name: &Token, value: EnvironmentValue) -> Result<(), Error> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = self.enclosing.as_mut() {
            enclosing.assign(name, value)?;
            return Ok(());
        }

        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }
}
