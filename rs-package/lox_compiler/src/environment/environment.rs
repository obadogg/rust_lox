use super::super::scanner::{scanner::*, tokens::*};
use crate::environment::environment_value::*;
use crate::semantic::scope_analyst::*;

use std::collections::BTreeMap;

// #[derive(Debug, Clone)]
// pub struct Environment {
//     pub values: BTreeMap<*const u8, EnvironmentValue>,
//     pub enclosing: Option<Rc<RefCell<Environment>>>,
// }

// impl Environment {
//     pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
//         Environment {
//             values: BTreeMap::new(),
//             enclosing,
//         }
//     }

//     pub fn define(&mut self, name: *const u8, value: EnvironmentValue) {
//         self.values.insert(name, value);
//     }

//     pub fn get(&self, name: &Token) -> Result<EnvironmentValue, Error> {
//         let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
//         if self.values.contains_key(&name_ptr) {
//             return Ok(self.values.get(&name_ptr).unwrap().clone());
//         }

//         if let Some(ref enclosing) = self.enclosing {
//             return enclosing.borrow_mut().get(name);
//         }
//         Err(Error {
//             line: name.line,
//             column: name.column,
//             message: String::from("Undefined variable at ") + name.lexeme.as_str(),
//         })
//     }

//     pub fn get_env_by_distance(
//         env: Rc<RefCell<Environment>>,
//         distance: usize,
//     ) -> Rc<RefCell<Environment>> {
//         let mut environment = env;
//         let mut distance = distance;

//         loop {
//             if distance > 0 {
//                 let e = environment.clone();
//                 let ee = e.borrow_mut();
//                 if let Some(ref enclosing) = ee.enclosing {
//                     environment = enclosing.clone();
//                     distance -= 1;
//                 } else {
//                     break;
//                 }
//             } else {
//                 break;
//             }
//         }
//         environment
//     }

//     pub fn assign(&mut self, name: &Token, value: EnvironmentValue) -> Result<(), Error> {
//         let name_ptr = name.lexeme.as_ptr();
//         if self.values.contains_key(&name_ptr) {
//             self.define(name_ptr, value);
//             return Ok(());
//         }

//         if let Some(ref enclosing) = self.enclosing {
//             enclosing.borrow_mut().assign(name, value)?;
//             return Ok(());
//         }

//         Err(Error {
//             line: name.line,
//             column: name.column,
//             message: String::from("Undefined variable at ") + name.lexeme.as_str(),
//         })
//     }
// }
#[derive(Debug, Clone)]
pub struct EnvironmentList {
    list: Vec<BTreeMap<*const u8, EnvironmentValue>>,
    pub env_pos: usize,
    pub previous_vec: Vec<Option<usize>>,
}

impl EnvironmentList {
    pub fn new() -> Self {
        let list = vec![BTreeMap::new()];
        Self {
            list,
            env_pos: 0,
            previous_vec: vec![None],
        }
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

    pub fn get_by_distance(
        &self,
        name: &Token,
        distance: usize,
    ) -> Result<&EnvironmentValue, Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
        let value = self.get_by_distance_default(&name_ptr, distance);

        if value.is_err() {
            Err(Error {
                line: name.line,
                column: name.column,
                message: String::from("Undefined variable at ") + name.lexeme.as_str(),
            })
        } else {
            Ok(value.unwrap())
        }
    }

    pub fn get_by_distance_default(
        &self,
        name_ptr: &*const u8,
        distance: usize,
    ) -> Result<&EnvironmentValue, ()> {
        let mut env_pos = self.env_pos;

        for _ in 0..distance {
            env_pos = if let Some(previous_pos) = self.previous_vec.get(env_pos).unwrap() {
                *previous_pos
            } else {
                break;
            }
        }
        if let Some(env) = self.list.get(env_pos) {
            let value = env.get(name_ptr);
            if let Some(value) = value {
                return Ok(value);
            }
        }
        Err(())
    }

    pub fn assign_by_distance(
        &mut self,
        name: &Token,
        distance: usize,
        value: EnvironmentValue,
    ) -> Result<(), Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);

        let mut env_pos = self.env_pos;

        for _ in 0..distance {
            env_pos = if let Some(previous_pos) = self.previous_vec.get(env_pos).unwrap() {
                *previous_pos
            } else {
                break;
            }
        }

        if let Some(env) = self.list.get_mut(env_pos) {
            env.insert(name_ptr, value);
        }
        Ok(())
    }

    pub fn assign(&mut self, name: &Token, value: EnvironmentValue) -> Result<(), Error> {
        let name_ptr = name.lexeme.as_ptr();
        let mut env;

        let mut env_pos = self.env_pos;

        loop {
            env = self.list.get_mut(env_pos);

            if let Some(env) = env {
                if env.contains_key(&name_ptr) {
                    env.insert(name_ptr, value);
                    return Ok(());
                }
            }

            env_pos = if let Some(previous_pos) = self.previous_vec.get(env_pos).unwrap() {
                *previous_pos
            } else {
                break;
            };
        }

        Err(Error {
            line: name.line,
            column: name.column,
            message: String::from("Undefined variable at ") + name.lexeme.as_str(),
        })
    }

    pub fn next(&mut self, previous: Option<usize>) -> usize {
        let from_pos = self.env_pos;
        let previous = if let Some(previous) = previous {
            previous
        } else {
            self.env_pos
        };

        for pos in self.env_pos + 1..self.list.len() - 1 {
            match (self.list.get(pos), self.previous_vec.get(pos).unwrap()) {
                (Some(_), None) => {
                    self.previous_vec[pos] = Some(previous);
                    self.env_pos = pos;
                    return from_pos;
                }
                (_, _) => {}
            }
        }

        self.list.push(BTreeMap::new());
        self.previous_vec.push(None);

        let pos = self.list.len() - 1;
        self.previous_vec[pos] = Some(previous);
        self.env_pos = pos;
        from_pos
    }

    pub fn back(&mut self) {
        let previous_pos = self.previous_vec.get(self.env_pos).unwrap().unwrap();
        self.list.get_mut(self.env_pos).unwrap().clear();
        self.previous_vec[self.env_pos] = None;
        self.env_pos = previous_pos;
    }

    pub fn go_to_env_by_pos(&mut self, env_pos: usize) {
        self.env_pos = env_pos;
    }

    pub fn global_get(&self, name: &Token) -> Result<&EnvironmentValue, Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
        let value = self.list.get(0).unwrap().get(&name_ptr);

        if let Some(value) = value {
            Ok(value)
        } else {
            Err(Error {
                line: name.line,
                column: name.column,
                message: String::from("Undefined variable at ") + name.lexeme.as_str(),
            })
        }
    }

    pub fn global_assign(&mut self, name: &Token, value: EnvironmentValue) -> Result<(), Error> {
        let name_ptr = ScopeAnalyst::get_scope_key_name(&name.lexeme);
        self.list.get_mut(0).unwrap().insert(name_ptr, value);
        Ok(())
    }

    pub fn get_with_pos(
        &mut self,
        name_ptr: &*const u8,
        pos: usize,
    ) -> Result<&EnvironmentValue, Error> {
        Ok(self.list.get(pos).unwrap().get(name_ptr).unwrap())
    }
}
