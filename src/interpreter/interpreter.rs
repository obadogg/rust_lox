use crate::environment::{environment::*, environment_value::*};
use crate::parser::{expression::*, statement::*};
use crate::scanner::{scanner::Error, tokens::*};
use crate::utils::utils::get_rc_ref_address;

use super::lox_function::LoxFunction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    statements: Rc<Vec<Stmt>>,
    scope_record: Rc<RefCell<HashMap<usize, usize>>>,
}

impl Interpreter {
    pub fn new(
        statements: Rc<Vec<Stmt>>,
        scope_record: Rc<RefCell<HashMap<usize, usize>>>,
    ) -> Self {
        let env = Rc::new(RefCell::new(Environment::new(None)));
        Interpreter {
            global: env.clone(),
            environment: env.clone(),
            statements,
            scope_record,
        }
    }

    pub fn interpret(&mut self) {
        for stmt in self.statements.clone().iter() {
            self.evaluate_statement_item(stmt);
        }
    }

    fn evaluate_expression_item(&mut self, expr: &Expr) -> Result<EnvironmentValue, Error> {
        match expr {
            Expr::Binary(expr_binary) => self.visit_binary_expr(expr_binary),
            Expr::Logical(expr_logical) => self.visit_logical_expr(expr_logical),
            Expr::Grouping(expr_grouping) => self.visit_grouping_expr(expr_grouping),
            Expr::Literal(expr_literal) => self.visit_literal_expr(expr_literal),
            Expr::Unary(expr_unary) => self.visit_unary_expr(expr_unary),
            Expr::Variable(expr_variable) => self.visit_variable_expr(expr_variable),
            Expr::Assignment(expr_assignment) => self.visit_assignment_expr(expr_assignment),
            Expr::Call(expr_call) => self.visit_call_expr(expr_call),
            // Expr::Get(expr_get) => self.visit_get_expr(expr_get),
            // Expr::Set(expr_set) => self.visit_set_expr(expr_set),
            // Expr::This(expr_this) => self.visit_this_expr(expr_this),
            // Expr::Super(expr_super) => self.visit_super_expr(expr_super),
            _ => Err(Error {
                line: 1,
                column: 1,
                message: String::from("asdas"),
            }),
        }
    }

    fn evaluate_statement_item(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Function(stmt_function) => self.visit_function_stmt(stmt_function),
            Stmt::If(stmt_if) => self.visit_if_stmt(stmt_if),
            Stmt::Expression(stmt_expression) => self.visit_expression_stmt(stmt_expression),
            Stmt::Print(stmt_print) => self.visit_print_stmt(stmt_print),
            Stmt::While(stmt_while) => self.visit_while_stmt(stmt_while),
            Stmt::For(stmt_for) => self.visit_for_stmt(stmt_for),
            Stmt::Var(stmt_var) => self.visit_var_stmt(stmt_var),
            Stmt::Block(stmt_block) => self.visit_block_stmt(stmt_block, None),
            Stmt::Return(stmt_return) => self.visit_return_stmt(stmt_return),
            Stmt::Class(stmt_class) => self.visit_class_stmt(stmt_class),
            _ => Ok(()),
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Rc<FunctionStatement>) -> Result<(), Error> {
        let lox_function = LoxFunction::new(stmt.clone(), self.environment.clone(), false);
        self.environment.borrow_mut().define(
            stmt.name.lexeme.clone(),
            EnvironmentValue::LoxFunction(lox_function),
        );
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> Result<(), Error> {
        let value = self.evaluate_expression_item(&stmt.condition)?;

        if value.is_truthy() {
            self.evaluate_statement_item(&stmt.then_branch);
        } else {
            if let Some(else_branch) = &stmt.else_branch {
                self.evaluate_statement_item(else_branch);
            }
        }

        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStatement) -> Result<(), Error> {
        self.evaluate_expression_item(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStatement) -> Result<(), Error> {
        let val = self.evaluate_expression_item(&stmt.expression)?;
        //TODO:
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStatement) -> Result<(), Error> {
        while self.evaluate_expression_item(&stmt.condition)?.is_truthy() {
            self.evaluate_statement_item(&stmt.body)?;
        }

        Ok(())
    }

    fn visit_for_stmt(&mut self, stmt: &ForStatement) -> Result<(), Error> {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(Environment::new(Some(previous))));

        if let Some(initializer) = &stmt.initializer {
            self.evaluate_statement_item(initializer)?;
        }

        while stmt.condition.is_some()
            && self
                .evaluate_expression_item(stmt.condition.as_ref().unwrap())?
                .is_truthy()
        {
            self.evaluate_statement_item(&stmt.body)?;

            if let Some(updator) = &stmt.updator {
                self.evaluate_expression_item(updator)?;
            }
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStatement) -> Result<(), Error> {
        let mut value = EnvironmentValue::None;
        if let Some(initializer) = &stmt.initializer {
            value = self.evaluate_expression_item(initializer)?;
        }
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    pub fn visit_block_stmt(
        &mut self,
        stmt: &BlockStatement,
        environment: Option<Rc<RefCell<Environment>>>,
    ) -> Result<(), Error> {
        let previous = self.environment.clone();

        if let Some(new_env) = environment {
            self.environment = new_env;
        } else {
            self.environment = Rc::new(RefCell::new(Environment::new(Some(previous.clone()))))
        }

        let stmts = &stmt.statements;

        for statement in stmts.iter() {
            if let Err(err) = self.evaluate_statement_item(statement) {
                self.environment = previous.clone();
                return Err(err);
            }
        }
        self.environment = previous.clone();
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> Result<(), Error> {
        self.evaluate_expression_item(&stmt.value)?;
        //TODO:
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStatement) -> Result<(), Error> {
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), EnvironmentValue::None);

        if let Some(superclass) = &stmt.superclass {
            let superclass_value = self.evaluate_expression_item(superclass)?;

            match superclass_value {
                EnvironmentValue::LoxClass(superclass_value_lox_class) => {}
                _ => {
                    return Err(Error {
                        line: stmt.name.line,
                        column: stmt.name.column,
                        message: String::from("Superclass must be a class at ")
                            + stmt.name.lexeme.as_str(),
                    })
                }
            };
        }

        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpression) -> Result<EnvironmentValue, Error> {
        let left = self.evaluate_expression_item(&expr.left)?;
        let right = self.evaluate_expression_item(&expr.right)?;

        match expr.operator.token_type {
            TokensType::Plus => {
                if let Ok(value) = left + right {
                    return Ok(value);
                } else {
                    Err(Error {
                        line: expr.operator.line,
                        column: expr.operator.column,
                        message: String::from("Operands must be two numbers or two strings ")
                            + &*expr.operator.lexeme,
                    })
                }
            }
            TokensType::Minus
            | TokensType::Slash
            | TokensType::Star
            | TokensType::Greater
            | TokensType::GreaterEqual
            | TokensType::Less
            | TokensType::LessEqual => {
                return Ok(self.number_binary_calculate(&expr.operator, left, right)?)
            } //TODO:
            // TokensType::BangEqual =>
            // TokensType::EqualEqual =>
            _ => Err(Error {
                line: expr.operator.line,
                column: expr.operator.column,
                message: String::from("Should not happen"),
            }),
        }
    }

    fn number_binary_calculate(
        &mut self,
        operator: &Token,
        left: EnvironmentValue,
        right: EnvironmentValue,
    ) -> Result<EnvironmentValue, Error> {
        let (left_is_number, _) = left.is_number();
        let (right_is_number, _) = right.is_number();

        if !left_is_number || !right_is_number {
            return Err(Error {
                line: operator.line,
                column: operator.column,
                message: String::from("Operands must be two numbers or two strings ")
                    + &*operator.lexeme,
            });
        }

        match operator.token_type {
            TokensType::Minus => Ok((left - right).unwrap()),
            TokensType::Slash => Ok((left / right).unwrap()),
            TokensType::Star => Ok((left * right).unwrap()),
            TokensType::Greater => Ok((left.gt(right)).unwrap()),
            TokensType::GreaterEqual => Ok((left.ge(right)).unwrap()),
            TokensType::Less => Ok((left.lt(right)).unwrap()),
            TokensType::LessEqual => Ok((left.le(right)).unwrap()),
            _ => Err(Error {
                line: operator.line,
                column: operator.column,
                message: String::from("Should not happen"),
            }),
        }
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpression) -> Result<EnvironmentValue, Error> {
        let left = self.evaluate_expression_item(&expr.left)?;

        match expr.operator.token_type {
            TokensType::Or => {
                if left.is_truthy() {
                    return Ok(EnvironmentValue::Bool(true));
                }
                return self.evaluate_expression_item(&expr.right);
            }
            TokensType::And => {
                if left.is_truthy() {
                    return self.evaluate_expression_item(&expr.right);
                }
                return Ok(EnvironmentValue::Bool(false));
            }
            _ => Err(Error {
                line: expr.operator.line,
                column: expr.operator.column,
                message: String::from("Should not happen"),
            }),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &GroupingExpression,
    ) -> Result<EnvironmentValue, Error> {
        self.evaluate_expression_item(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpression) -> Result<EnvironmentValue, Error> {
        let ref value = expr.value;

        if value.is_none() {
            Ok(EnvironmentValue::None)
        } else {
            match &value.clone().unwrap() {
                ValueType::Bool(bool_val) => Ok(EnvironmentValue::Bool(*bool_val)),
                ValueType::Number(number_val) => Ok(EnvironmentValue::Number(*number_val)),
                ValueType::String(string_val) => Ok(EnvironmentValue::String(string_val.clone())),
            }
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpression) -> Result<EnvironmentValue, Error> {
        let right = self.evaluate_expression_item(&expr.expression)?;

        match expr.operator.token_type {
            TokensType::Minus => {
                let (right_is_number, _) = right.is_number();
                if !right_is_number {
                    return Err(Error {
                        line: expr.operator.line,
                        column: expr.operator.column,
                        message: String::from("Operand must be a number at ")
                            + expr.operator.lexeme.as_str(),
                    });
                }
                return Ok((-right).unwrap());
            }
            TokensType::Bang => Ok(EnvironmentValue::Bool(!right.is_truthy())),
            _ => Err(Error {
                line: expr.operator.line,
                column: expr.operator.column,
                message: String::from("Should not happen"),
            }),
        }
    }

    fn visit_variable_expr(
        &mut self,
        expr: &Rc<VariableExpression>,
    ) -> Result<EnvironmentValue, Error> {
        let add = get_rc_ref_address(expr.clone());

        if !self.scope_record.borrow().contains_key(&add) {
            return self.global.borrow().get(&expr.name.clone());
        }
        let borrow_env = self.scope_record.borrow();
        let distance = borrow_env.get(&add).unwrap();
        let ref environment = self.environment;
        let environment = Environment::get_env_by_distance(environment.clone(), *distance);
        let value = environment.borrow().get(&expr.name);
        value
    }

    fn visit_assignment_expr(
        &mut self,
        expr: &Rc<AssignmentExpression>,
    ) -> Result<EnvironmentValue, Error> {
        let value = self.evaluate_expression_item(&expr.value)?;
        let add = get_rc_ref_address(expr.clone());

        if !self.scope_record.borrow().contains_key(&add) {
            self.global.borrow_mut().assign(&expr.name, value.clone())?;
            return Ok(value);
        }

        let borrow_env = self.scope_record.borrow();
        let distance = borrow_env.get(&add).unwrap();
        let ref environment = self.environment;
        let environment = Environment::get_env_by_distance(environment.clone(), *distance);

        environment.borrow_mut().assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_call_expr(&mut self, expr: &CallExpression) -> Result<EnvironmentValue, Error> {
        let mut callee = self.evaluate_expression_item(&expr.callee)?;
        let args = &expr
            .args
            .iter()
            .map(|arg| self.evaluate_expression_item(arg))
            .collect::<Vec<_>>();

        match callee {
            // EnvironmentValue::LoxClass(lox_class) => {}
            EnvironmentValue::LoxFunction(ref mut lox_function) => {
                if args.len() != lox_function.arity() {
                    return Err(Error {
                        line: expr.end_parenthese.line,
                        column: expr.end_parenthese.column,
                        message: String::from("Expect ")
                            + lox_function.arity().to_string().as_str()
                            + " arguments but got "
                            + args.len().to_string().as_str()
                            + " at \")\"",
                    });
                }
                return lox_function.call(self, args);
            }
            _ => {
                return Err(Error {
                    line: expr.end_parenthese.line,
                    column: expr.end_parenthese.column,
                    message: String::from("Can only call functions and classes at ")
                        + &*expr.end_parenthese.lexeme,
                })
            }
        }
    }
}
