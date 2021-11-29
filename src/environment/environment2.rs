use super::super::scanner::{scanner::*, tokens::*};
use super::environment_value::*;
use crate::semantic::scope_analyst::*;
use std::collections::BTreeMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: BTreeMap<*const u8, EnvironmentValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: BTreeMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: *const u8, value: EnvironmentValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<EnvironmentValue, Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
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

#[derive(Debug, Clone)]
struct TupleMap {
    vec: Vec<(*const u8, EnvironmentValue)>,
}

#[derive(Debug, Clone)]
pub struct EnvironmentList {
    list: Vec<BTreeMap<*const u8, EnvironmentValue>>,
    pub env_pos: usize,
}

impl EnvironmentList {
    pub fn new() -> Self {
        let list = vec![BTreeMap::new()];
        Self { list, env_pos: 0 }
    }

    pub fn define(
        &mut self,
        name_ptr: *const u8,
        value: EnvironmentValue,
    ) -> Result<&EnvironmentValue, Error> {
        let env = self.list.get_mut(self.env_pos).unwrap();
        env.insert(name_ptr, value);
        Ok(env.get(&name_ptr).unwrap())
    }

    // pub fn get(&self, name: &Token) -> Result<&EnvironmentValue, Error> {
    //     let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);

    //     for pos in 0..self.env_pos {
    //         if let Some(env) = self.list.get(pos) {
    //             if env.contains_key(&name_ptr) {
    //                 return Ok(env.get(&name_ptr).unwrap());
    //             }
    //         }
    //     }

    //     Err(Error {
    //         line: name.line,
    //         column: name.column,
    //         message: String::from("Undefined variable at ") + name.lexeme.as_str(),
    //     })
    // }

    pub fn get_by_distance(
        &self,
        name: &Token,
        distance: usize,
    ) -> Result<&EnvironmentValue, Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);

        if let Some(env) = self.list.get(self.env_pos - distance) {
            let value = env.get(&name_ptr);
            if let Some(value) = value {
                return Ok(value);
            }
            // return Ok(&EnvironmentValue::Number(10000_f64));
        }
        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }

    pub fn assign_by_distance(
        &mut self,
        name: &Token,
        distance: usize,
        value: EnvironmentValue,
    ) -> Result<(), Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);

        if let Some(env) = self.list.get_mut(self.env_pos - distance) {
            env.insert(name_ptr, value);
        }
        Ok(())
    }

    pub fn assign(&mut self, name: &Token, value: EnvironmentValue) -> Result<(), Error> {
        let name_ptr = name.lexeme.as_ptr();
        let mut env = self.list.get_mut(self.env_pos);

        for pos in 0..self.env_pos {
            env = self.list.get_mut(pos);
            if let Some(env) = env {
                if env.contains_key(&name_ptr) {
                    env.insert(name_ptr, value);
                    return Ok(());
                }
            }
        }

        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }

    pub fn next(&mut self) {
        if self.list.get(self.env_pos + 1).is_none() {
            self.list.push(BTreeMap::new());
        }
        self.env_pos = self.env_pos + 1;
    }

    pub fn back(&mut self) {
        self.env_pos = self.env_pos - 1;
    }

    pub fn back_with_pop(&mut self) {
        self.env_pos = self.env_pos - 1;
        self.list.pop();
    }

    pub fn global_get(&self, name: &Token) -> Result<&EnvironmentValue, Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
        Ok(self.list.get(0).unwrap().get(&name_ptr).unwrap())
    }

    pub fn global_assign(&mut self, name: &Token, value: EnvironmentValue) -> Result<(), Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
        self.list.get_mut(0).unwrap().insert(name_ptr, value);
        Ok(())
    }
}
