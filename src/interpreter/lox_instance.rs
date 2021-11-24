use crate::environment::environment_value::EnvironmentValue;
use crate::scanner::{scanner::*, tokens::*};

use super::lox_class::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    belong_class: Rc<RefCell<LoxClass>>,
    fields: HashMap<Rc<String>, EnvironmentValue>,
}

impl LoxInstance {
    pub fn new(belong_class: Rc<RefCell<LoxClass>>) -> Self {
        LoxInstance {
            belong_class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<EnvironmentValue, Error> {
        if let Some(value) = self.fields.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(method) = self.belong_class.borrow().find_method(&name.lexeme) {
            let borrow_function = method.clone();
            let mut borrow_function = borrow_function.borrow_mut();
            return Ok(
                borrow_function.bind(EnvironmentValue::LoxInstance(Rc::new(RefCell::new(
                    self.clone(),
                )))),
            );
        }

        Err(Error {
            line: name.line,
            column: name.column,
            message: format!("Undefined property {}", &name.lexeme),
        })
    }

    pub fn set(&mut self, name: &Token, value: EnvironmentValue) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}
