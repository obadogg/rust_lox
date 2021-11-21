use crate::interpreter::lox_class::LoxClass;
use crate::interpreter::lox_function::LoxFunction;
use crate::interpreter::lox_instance::LoxInstance;
use crate::interpreter::lox_return::LoxReturn;

use super::super::scanner::{scanner::*, tokens::*};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    LoxClass(LoxClass),
    LoxFunction(LoxFunction),
    LoxInstance(LoxInstance),
    LoxReturn(LoxReturn),
    // LoxNativeFunction,
    // LoxNativeClass,
    LiteralValue(Option<ValueType>),
}

impl EnvironmentValue {
    pub fn is_truthy(&self) -> bool {
        let mut flag = false;
        match self {
            EnvironmentValue::LiteralValue(value) => {
                if let Some(val) = value {
                    match val {
                        ValueType::Number(num_val) => flag = *num_val != 0_f64,
                        ValueType::String(string_val) => flag = string_val.len() != 0,
                        ValueType::Bool(bool_val) => flag = *bool_val,
                    }
                }
            }
            _ => {}
        }
        flag
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Option<EnvironmentValue>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Option<EnvironmentValue>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Option<EnvironmentValue>, Error> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
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

    pub fn assign(&mut self, name: &Token, value: Option<EnvironmentValue>) -> Result<(), Error> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme.clone(), value);
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
