use crate::environment::environment_value::EnvironmentValue;

use super::super::environment::*;
use super::super::scanner::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    // belong_class:
    fields: HashMap<String, EnvironmentValue>,
}

impl LoxInstance {
    pub fn new() -> Self {
        LoxInstance {
            fields: HashMap::new(),
        }
    }
}
