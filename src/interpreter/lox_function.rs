use crate::environment::{environment::Environment, environment_value::EnvironmentValue};
use crate::interpreter::interpreter::Interpreter;
use crate::parser::statement::FunctionStatement;
use crate::scanner::scanner::Error;
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

    pub fn call(
        &mut self,
        interpreter: &mut Interpreter,
        args: &Vec<Result<EnvironmentValue, Error>>,
    ) -> Result<EnvironmentValue, Error> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));

        let iter = &self.declaration.clone().params;
        for (pos, decs) in iter.iter().enumerate() {
            let arg = args[pos].clone().unwrap();
            environment.borrow_mut().define(decs.lexeme.clone(), arg)
        }

        let return_val = EnvironmentValue::None;

        interpreter.visit_block_stmt(&self.declaration.body, Some(environment.clone()))?;
        //TODO:

        if self.is_initializer {
            let borrow = self.closure.borrow();
            let value = borrow.values.get(&String::from("this")).unwrap();

            return Ok(value.clone());
        }
        Ok(return_val)
    }
}
