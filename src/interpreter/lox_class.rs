use crate::environment::environment_value::EnvironmentValue;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::lox_function::*;
use crate::interpreter::lox_instance::*;
use crate::scanner::scanner::Error;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: Rc<String>,
    superclass: Option<Rc<RefCell<LoxClass>>>,
    methods: HashMap<Rc<String>, Rc<RefCell<LoxFunction>>>,
}

impl LoxClass {
    pub fn new(
        name: Rc<String>,
        superclass: Option<Rc<RefCell<LoxClass>>>,
        methods: HashMap<Rc<String>, Rc<RefCell<LoxFunction>>>,
    ) -> Self {
        LoxClass {
            name,
            superclass,
            methods,
        }
    }

    pub fn arity(&self) -> usize {
        if let Some(initializer) = self.methods.get(&String::from("init")) {
            return initializer.borrow().arity();
        }
        0
    }

    pub fn find_method(&self, name: &String) -> Option<Rc<RefCell<LoxFunction>>> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(superclass) = self.superclass.clone() {
            return superclass.clone().borrow().find_method(&name.clone());
        }

        return None;
    }

    pub fn call(
        &mut self,
        interpreter: &mut Interpreter,
        args: &Vec<Result<EnvironmentValue, Error>>,
    ) -> Result<EnvironmentValue, Error> {
        let instance = LoxInstance::new(Rc::new(RefCell::new(self.clone())));
        let instance = Rc::new(RefCell::new(instance));

        if let Some(initializer) = self.methods.get(&String::from("init")) {
            let mut borrow_function = initializer.borrow_mut();

            let value = borrow_function.bind(EnvironmentValue::LoxInstance(instance.clone()));

            match value {
                EnvironmentValue::LoxFunction(lox_function) => {
                    let borrow_lox_function = lox_function.clone();
                    let mut borrow_lox_function = borrow_lox_function.borrow_mut();
                    borrow_lox_function.call(interpreter, args)?;
                }
                _ => {}
            }
        }
        return Ok(EnvironmentValue::LoxInstance(instance.clone()));
    }
}
