use super::super::scanner::{scanner::*, tokens::*};
use super::environment_value::*;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<*const u8, EnvironmentValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: *const u8, value: EnvironmentValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<EnvironmentValue, Error> {
        let name_ptr = name.lexeme.as_ptr();
        if self.values.contains_key(&name_ptr) {
            return Ok(self.values.get(&name_ptr).unwrap().clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().get(name);
        }
        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }

    pub fn get_env_by_distance(
        env: Rc<RefCell<Environment>>,
        distance: usize,
    ) -> Rc<RefCell<Environment>> {
        let mut environment = env;
        let mut distance = distance;

        loop {
            if distance > 0 {
                let e = environment.clone();
                let ee = e.borrow_mut();
                if let Some(ref enclosing) = ee.enclosing {
                    environment = enclosing.clone();
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
        let name_ptr = name.lexeme.as_ptr();
        if self.values.contains_key(&name_ptr) {
            self.define(name_ptr, value);
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }
}
