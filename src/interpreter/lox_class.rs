use crate::interpreter::lox_function::*;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        LoxClass {
            name,
            superclass,
            methods,
        }
    }
}
