use crate::environment::environment::Environment;
use crate::parser::statement::FunctionStatement;

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Rc<FunctionStatement>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(
        declaration: Rc<FunctionStatement>,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        LoxFunction {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
