use crate::environment::environment_value::EnvironmentValue;
use crate::interpreter::interpreter::Interpreter;
use crate::parser::statement::FunctionStatement;
use crate::scanner::scanner::Error;
use crate::semantic::scope_analyst::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Rc<FunctionStatement>,
    closure: usize,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(declaration: Rc<FunctionStatement>, closure: usize, is_initializer: bool) -> Self {
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
        let iter = &self.declaration.clone().params;

        let env_pos = interpreter.envs.env_pos;
        interpreter.envs.next(Some(self.closure));
        for (pos, decs) in iter.iter().enumerate() {
            let arg = args[pos].clone().unwrap();
            let name_ptr = ScopeAnalyst::get_scope_key_name(&decs.lexeme);
            interpreter.envs.define(name_ptr, arg)?;
        }

        let block_previous_env_pos = interpreter.envs.env_pos;

        // interpreter.envs.back_without_clear();
        interpreter.envs.env_pos = env_pos;

        interpreter.visit_block_stmt(&self.declaration.body, Some(block_previous_env_pos))?;

        let return_val = interpreter.return_val.clone();

        if self.is_initializer {
            let value = interpreter
                .envs
                .get_with_pos(&THIS_STRING.as_ptr(), self.closure)?
                .clone();
            return Ok(value.clone());
        }
        Ok(return_val)
    }

    pub fn bind(
        &mut self,
        instance: EnvironmentValue,
        interpreter: &mut Interpreter,
    ) -> Result<EnvironmentValue, Error> {
        let from_env_pos = interpreter.envs.next(Some(self.closure));
        interpreter.envs.define(THIS_STRING.as_ptr(), instance)?;

        let lox_function = LoxFunction::new(
            self.declaration.clone(),
            interpreter.envs.env_pos,
            self.is_initializer,
        );
        interpreter.envs.go_to_env_by_pos(from_env_pos);

        return Ok(EnvironmentValue::LoxFunction(Rc::new(RefCell::new(
            lox_function,
        ))));
    }
}
