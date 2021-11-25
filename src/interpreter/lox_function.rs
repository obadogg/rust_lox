use crate::environment::{environment::*, environment_value::EnvironmentValue};
use crate::interpreter::interpreter::Interpreter;
use crate::parser::statement::FunctionStatement;
use crate::scanner::scanner::Error;
use crate::semantic::scope_analyst::*;
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
            let name_ptr = ScopeAnalyst::get_scope_key_name(&decs.lexeme);
            environment.borrow_mut().define(name_ptr, arg)
        }

        interpreter.visit_block_stmt(&self.declaration.body, Some(environment.clone()))?;

        let return_val = interpreter.return_val.clone();

        if self.is_initializer {
            let borrow = self.closure.borrow();
            let value = borrow.values.get(&THIS_STRING.as_ptr()).unwrap();

            return Ok(value.clone());
        }
        Ok(return_val)
    }

    pub fn bind(&mut self, instance: EnvironmentValue) -> EnvironmentValue {
        let environment = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));
        environment
            .borrow_mut()
            .define(THIS_STRING.as_ptr(), instance);

        let lox_function =
            LoxFunction::new(self.declaration.clone(), environment, self.is_initializer);
        return EnvironmentValue::LoxFunction(Rc::new(RefCell::new(lox_function)));
    }
}
