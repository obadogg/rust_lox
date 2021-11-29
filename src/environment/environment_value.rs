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

    pub fn is_number(&self) -> bool {
        match self {
            EnvironmentValue::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            EnvironmentValue::String(_) => true,
            _ => false,
        }
    }

    // The lt, le, gt, and ge methods of this trait can be called using the <, <=, >, and >= operators, respectively.
    #[inline]
    pub fn lt(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Bool(left < right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn le(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Bool(left <= right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn gt(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Bool(left > right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn ge(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Bool(left >= right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn eq(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Bool(left == right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn partial_eq(
        lhs: &EnvironmentValue,
        rhs: &EnvironmentValue,
    ) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Bool(left != right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn add(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Number(left + right))
            }
            (EnvironmentValue::String(left), EnvironmentValue::String(right)) => {
                return Ok(EnvironmentValue::String(left.to_owned() + &*right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn sub(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Number(left - right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn div(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Number(left / right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn mul(lhs: &EnvironmentValue, rhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match (lhs, rhs) {
            (EnvironmentValue::Number(left), EnvironmentValue::Number(right)) => {
                return Ok(EnvironmentValue::Number(left * right))
            }
            (_, _) => Err(()),
        }
    }

    #[inline]
    pub fn neg(lhs: &EnvironmentValue) -> Result<EnvironmentValue, ()> {
        match lhs {
            EnvironmentValue::Number(left) => return Ok(EnvironmentValue::Number(-left)),
            _ => Err(()),
        }
    }
}

// 不使用运算符重载
// impl ops::Add<Self> for EnvironmentValue {
//     type Output = Result<EnvironmentValue, ()>;

//     fn add(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
//         let (left_is_number, left_number) = self.is_number();
//         let (left_is_string, left_string) = self.is_string();
//         let (right_is_number, right_number) = rhs.is_number();
//         let (right_is_string, right_string) = rhs.is_string();

//         if left_is_number && right_is_number {
//             return Ok(EnvironmentValue::Number(
//                 left_number.unwrap() + right_number.unwrap(),
//             ));
//         }

//         if left_is_string && right_is_string {
//             return Ok(EnvironmentValue::String(
//                 left_string.unwrap() + &*right_string.unwrap(),
//             ));
//         }
//         Err(())
//     }
// }

// impl ops::Sub<Self> for EnvironmentValue {
//     type Output = Result<EnvironmentValue, ()>;

//     fn sub(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
//         let (left_is_number, left_number) = self.is_number();
//         let (right_is_number, right_number) = rhs.is_number();

//         if left_is_number && right_is_number {
//             return Ok(EnvironmentValue::Number(
//                 left_number.unwrap() - right_number.unwrap(),
//             ));
//         }
//         Err(())
//     }
// }

// impl ops::Div<Self> for EnvironmentValue {
//     type Output = Result<EnvironmentValue, ()>;

//     fn div(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
//         let (left_is_number, left_number) = self.is_number();
//         let (right_is_number, right_number) = rhs.is_number();

//         if left_is_number && right_is_number {
//             return Ok(EnvironmentValue::Number(
//                 left_number.unwrap() / right_number.unwrap(),
//             ));
//         }
//         Err(())
//     }
// }

// impl ops::Mul<Self> for EnvironmentValue {
//     type Output = Result<EnvironmentValue, ()>;

//     fn mul(self, rhs: EnvironmentValue) -> Result<EnvironmentValue, ()> {
//         let (left_is_number, left_number) = self.is_number();
//         let (right_is_number, right_number) = rhs.is_number();

//         if left_is_number && right_is_number {
//             return Ok(EnvironmentValue::Number(
//                 left_number.unwrap() * right_number.unwrap(),
//             ));
//         }
//         Err(())
//     }
// }

// impl ops::Neg for EnvironmentValue {
//     type Output = Result<EnvironmentValue, ()>;

//     fn neg(self) -> Result<EnvironmentValue, ()> {
//         let (left_is_number, left_number) = self.is_number();

//         if left_is_number {
//             return Ok(EnvironmentValue::Number(-left_number.unwrap()));
//         }
//         Err(())
//     }
// }
