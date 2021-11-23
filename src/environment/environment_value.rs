// use super::super::scanner::{scanner::*, tokens::*};
use crate::interpreter::lox_class::LoxClass;
use crate::interpreter::lox_function::LoxFunction;
use crate::interpreter::lox_instance::LoxInstance;

use std::{cell::RefCell, ops, rc::Rc};

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    LoxClass(Rc<RefCell<LoxClass>>),
    LoxFunction(Rc<RefCell<LoxFunction>>),
    LoxInstance(Rc<RefCell<LoxInstance>>),
    // LoxNativeFunction,
    // LoxNativeClass,
    Number(f64),
    String(String),
    Bool(bool),
    None,
}

impl EnvironmentValue {
    pub fn is_truthy(&self) -> bool {
        let mut flag = true;
        match self {
            EnvironmentValue::Number(num_val) => flag = *num_val != 0_f64,
            EnvironmentValue::String(string_val) => flag = string_val.len() != 0,
            EnvironmentValue::Bool(bool_val) => flag = *bool_val,
            EnvironmentValue::None => flag = false,
            _ => {}
        }
        flag
    }

    pub fn as_print(&self) -> Result<String, ()> {
        match self {
            EnvironmentValue::Number(num_val) => Ok(num_val.to_string()),
            EnvironmentValue::String(string_val) => Ok(string_val.clone()),
            EnvironmentValue::Bool(bool_val) => Ok(bool_val.to_string()),
            EnvironmentValue::None => Ok(String::from("Nil")),
            _ => Err(()),
        }
    }

    pub fn is_number(&self) -> (bool, Option<f64>) {
        match self {
            EnvironmentValue::Number(number_val) => (true, Some(*number_val)),
            _ => (false, None),
        }
    }

    pub fn is_string(&self) -> (bool, Option<String>) {
        match self {
            EnvironmentValue::String(string_val) => (true, Some(string_val.clone())),
            _ => (false, None),
        }
    }

    // The lt, le, gt, and ge methods of this trait can be called using the <, <=, >, and >= operators, respectively.
    pub fn lt(&self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Bool(
                left_number.unwrap() < right_number.unwrap(),
            ));
        }
        Err(())
    }

    pub fn le(&self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Bool(
                left_number.unwrap() <= right_number.unwrap(),
            ));
        }
        Err(())
    }

    pub fn gt(&self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Bool(
                left_number.unwrap() > right_number.unwrap(),
            ));
        }
        Err(())
    }

    pub fn ge(&self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Bool(
                left_number.unwrap() >= right_number.unwrap(),
            ));
        }
        Err(())
    }

    pub fn eq(&self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Bool(
                left_number.unwrap() == right_number.unwrap(),
            ));
        }
        Err(())
    }

    pub fn partial_eq(&self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Bool(
                left_number.unwrap() != right_number.unwrap(),
            ));
        }
        Err(())
    }
}

impl ops::Add<Self> for EnvironmentValue {
    type Output = Result<EnvironmentValue, ()>;

    fn add(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (left_is_string, left_string) = self.is_string();
        let (right_is_number, right_number) = rhs.is_number();
        let (right_is_string, right_string) = rhs.is_string();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Number(
                left_number.unwrap() + right_number.unwrap(),
            ));
        }

        if left_is_string && right_is_string {
            return Ok(EnvironmentValue::String(
                left_string.unwrap() + &*right_string.unwrap(),
            ));
        }
        Err(())
    }
}

impl ops::Sub<Self> for EnvironmentValue {
    type Output = Result<EnvironmentValue, ()>;

    fn sub(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Number(
                left_number.unwrap() - right_number.unwrap(),
            ));
        }
        Err(())
    }
}

impl ops::Div<Self> for EnvironmentValue {
    type Output = Result<EnvironmentValue, ()>;

    fn div(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Number(
                left_number.unwrap() / right_number.unwrap(),
            ));
        }
        Err(())
    }
}

impl ops::Mul<Self> for EnvironmentValue {
    type Output = Result<EnvironmentValue, ()>;

    fn mul(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();
        let (right_is_number, right_number) = rhs.is_number();

        if left_is_number && right_is_number {
            return Ok(EnvironmentValue::Number(
                left_number.unwrap() * right_number.unwrap(),
            ));
        }
        Err(())
    }
}

impl ops::Neg for EnvironmentValue {
    type Output = Result<EnvironmentValue, ()>;

    fn neg(self) -> Result<EnvironmentValue, ()> {
        let (left_is_number, left_number) = self.is_number();

        if left_is_number {
            return Ok(EnvironmentValue::Number(-left_number.unwrap()));
        }
        Err(())
    }
}
