use super::super::scanner::{scanner::*, tokens::*};
use crate::interpreter::lox_class::LoxClass;
use crate::interpreter::lox_function::LoxFunction;
use crate::interpreter::lox_instance::LoxInstance;
use crate::interpreter::lox_return::LoxReturn;

use std::ops;

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    LoxClass(LoxClass),
    LoxFunction(LoxFunction),
    LoxInstance(LoxInstance),
    LoxReturn(LoxReturn),
    // LoxNativeFunction,
    // LoxNativeClass,
    LiteralValue(Option<ValueType>),
}

#[derive(Debug, Clone)]
pub enum AddValue {
    Number(f64),
    String(String),
}

impl EnvironmentValue {
    pub fn is_truthy(&self) -> bool {
        let mut flag = false;
        match self {
            EnvironmentValue::LiteralValue(value) => {
                if let Some(val) = value {
                    match val {
                        ValueType::Number(num_val) => flag = *num_val != 0_f64,
                        ValueType::String(string_val) => flag = string_val.len() != 0,
                        ValueType::Bool(bool_val) => flag = *bool_val,
                    }
                }
            }
            _ => {}
        }
        flag
    }
}

impl std::ops::Add<EnvironmentValue> for EnvironmentValue {
    type Output = Option<AddValue>;

    fn add(self, rhs: EnvironmentValue) -> Option<AddValue> {
        match self {
            EnvironmentValue::LiteralValue(value) => {
                if let Some(val) = value {
                    match val {
                        ValueType::Number(num_val) => {}
                    }
                }
                return None;
            }
            _ => None,
        }
    }
}
