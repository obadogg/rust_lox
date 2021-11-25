use crate::environment::{environment::*, environment_value::*};
use crate::parser::{expression::*, statement::*};
use crate::scanner::{scanner::Error, tokens::*};
use crate::utils::utils::get_rc_ref_address;

use super::lox_class::*;
use super::lox_function::{LoxFunction, SUPER_STRING, THIS_STRING};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    statements: Rc<Vec<Stmt>>,
    scope_record: Rc<RefCell<HashMap<usize, usize>>>,
    pub return_val: EnvironmentValue,
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
            return_val: EnvironmentValue::None,
        }
    }

    pub fn interpret(&mut self) {
        for stmt in self.statements.clone().iter() {
            self.evaluate_statement_item(stmt)
                .expect("oops! program panic:");
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
            Expr::Get(expr_get) => self.visit_get_expr(expr_get),
            Expr::Set(expr_set) => self.visit_set_expr(expr_set),
            Expr::This(expr_this) => self.visit_this_expr(expr_this),
            Expr::Super(expr_super) => self.visit_super_expr(expr_super),
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
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Rc<FunctionStatement>) -> Result<(), Error> {
        let lox_function = LoxFunction::new(stmt.clone(), self.environment.clone(), false);
        self.environment.borrow_mut().define(
            stmt.name.lexeme.as_ptr(),
            EnvironmentValue::LoxFunction(Rc::new(RefCell::new(lox_function))),
        );
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> Result<(), Error> {
        let value = self.evaluate_expression_item(&stmt.condition)?;

        if value.is_truthy() {
            self.evaluate_statement_item(&stmt.then_branch)?;
        } else {
            if let Some(else_branch) = &stmt.else_branch {
                self.evaluate_statement_item(else_branch)?;
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

        if let Ok(message) = val.as_print() {
            println!("{}", message);
            Ok(())
        } else {
            Err(Error {
                line: stmt.keyword.line,
                column: stmt.keyword.column,
                message: String::from("Lox only support print String/Boolean/Number"),
            })
        }
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

        if stmt.condition.is_some() {
            while self
                .evaluate_expression_item(stmt.condition.as_ref().unwrap())?
                .is_truthy()
            {
                self.evaluate_statement_item(&stmt.body)?;

                if let Some(updator) = &stmt.updator {
                    self.evaluate_expression_item(updator)?;
                }
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
            .define(stmt.name.lexeme.as_ptr(), value);
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
        self.return_val = self.evaluate_expression_item(&stmt.value)?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &Rc<ClassStatement>) -> Result<(), Error> {
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.as_ptr(), EnvironmentValue::None);

        let mut previous: Option<Rc<RefCell<Environment>>> = None;
        let mut super_class = None;

        if let Some(superclass) = &stmt.superclass {
            let superclass_value = self.evaluate_expression_item(superclass)?;

            match superclass_value {
                EnvironmentValue::LoxClass(superclass_value_lox_class) => {
                    previous = Some(self.environment.clone());
                    self.environment = Rc::new(RefCell::new(Environment::new(Some(
                        self.environment.clone(),
                    ))));
                    self.environment.borrow_mut().define(
                        SUPER_STRING.as_ptr(),
                        EnvironmentValue::LoxClass(superclass_value_lox_class.clone()),
                    );
                    super_class = Some(superclass_value_lox_class.clone());
                }
                _ => {
                    return Err(Error {
                        line: stmt.name.line,
                        column: stmt.name.column,
                        message: format!("Superclass must be a class at {}", &stmt.name.lexeme),
                    })
                }
            };
        }

        let methods = stmt.methods.clone();
        let methods = methods
            .iter()
            .map(|f_stmt| {
                let method = Rc::new(RefCell::new(LoxFunction::new(
                    f_stmt.clone(),
                    self.environment.clone(),
                    *f_stmt.name.lexeme == String::from("init"),
                )));
                (f_stmt.name.lexeme.clone(), method)
            })
            .collect::<HashMap<_, _>>();

        let lox_class = EnvironmentValue::LoxClass(Rc::new(RefCell::new(LoxClass::new(
            stmt.name.lexeme.clone(),
            super_class,
            methods,
        ))));

        if previous.is_some() {
            self.environment = previous.unwrap();
        }

        self.environment
            .borrow_mut()
            .assign(&stmt.name, lox_class)?;

        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpression) -> Result<EnvironmentValue, Error> {
        let left = self.evaluate_expression_item(&expr.left)?;
        let right = self.evaluate_expression_item(&expr.right)?;

        let err = Error {
            line: expr.operator.line,
            column: expr.operator.column,
            message: format!(
                "\"!=\" and \"==\" operands only support number/string/boolean {}",
                &expr.operator.lexeme
            ),
        };

        match expr.operator.token_type {
            TokensType::Plus => {
                if let Ok(value) = left + right {
                    return Ok(value);
                } else {
                    Err(Error {
                        line: expr.operator.line,
                        column: expr.operator.column,
                        message: format!(
                            "Operands must be two numbers or two strings {}",
                            &expr.operator.lexeme
                        ),
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
            }
            TokensType::BangEqual => {
                let result = left.partial_eq(right);
                if result.is_ok() {
                    return Ok(result.unwrap());
                }
                Err(err)
            }
            TokensType::EqualEqual => {
                let result = left.eq(right);
                if result.is_ok() {
                    return Ok(result.unwrap());
                }
                Err(err)
            }
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
                message: format!(
                    "Operands must be two numbers or two strings {}",
                    &operator.lexeme
                ),
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
        // let ref value = expr.value;

        if let Some(val) = &expr.value {
            match val {
                ValueType::Bool(bool_val) => Ok(EnvironmentValue::Bool(*bool_val)),
                ValueType::Number(number_val) => Ok(EnvironmentValue::Number(*number_val)),
                ValueType::String(string_val) => Ok(EnvironmentValue::String(string_val.clone())),
            }
        } else {
            Ok(EnvironmentValue::None)
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
                        message: format!("Operand must be a number at {}", &expr.operator.lexeme),
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
            return self.global.borrow().get(&expr.name);
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
            EnvironmentValue::LoxClass(ref mut lox_class) => {
                if args.len() != lox_class.borrow().arity() {
                    return Err(Error {
                        line: expr.end_parenthese.line,
                        column: expr.end_parenthese.column,
                        message: format!(
                            "Expect {} arguments but got {}, at \")\"",
                            lox_class.borrow().arity().to_string(),
                            args.len().to_string()
                        ),
                    });
                }
                return lox_class.borrow_mut().call(self, args);
            }
            EnvironmentValue::LoxFunction(ref mut lox_function) => {
                if args.len() != lox_function.borrow().arity() {
                    return Err(Error {
                        line: expr.end_parenthese.line,
                        column: expr.end_parenthese.column,
                        message: format!(
                            "Expect {} arguments but got {}, at \")\"",
                            lox_function.borrow().arity().to_string(),
                            args.len().to_string()
                        ),
                    });
                }
                return lox_function.borrow_mut().call(self, args);
            }
            _ => {
                return Err(Error {
                    line: expr.end_parenthese.line,
                    column: expr.end_parenthese.column,
                    message: format!(
                        "Can only call functions and classes at {}",
                        &expr.end_parenthese.lexeme
                    ),
                })
            }
        }
    }

    fn visit_get_expr(&mut self, expr: &GetExpression) -> Result<EnvironmentValue, Error> {
        let obj = self.evaluate_expression_item(&expr.object)?;

        match obj {
            EnvironmentValue::LoxInstance(lox_instance) => {
                let lox_instance = lox_instance.clone();
                let lox_instance = lox_instance.borrow_mut(); //TODO:实现的有没有问题？
                return lox_instance.get(&expr.name);
            }
            _ => Err(Error {
                line: expr.name.line,
                column: expr.name.column,
                message: format!("Only instances have properties at {}", &expr.name.lexeme),
            }),
        }
    }

    fn visit_set_expr(&mut self, expr: &SetExpression) -> Result<EnvironmentValue, Error> {
        let obj = self.evaluate_expression_item(&expr.object)?;

        match obj {
            EnvironmentValue::LoxInstance(lox_instance) => {
                let value = self.evaluate_expression_item(&expr.value)?;
                let mut lox_instance = lox_instance.borrow_mut();
                lox_instance.set(&expr.name, value.clone());
                return Ok(value);
            }
            _ => Err(Error {
                line: expr.name.line,
                column: expr.name.column,
                message: format!("Only instances have properties at {}", &expr.name.lexeme),
            }),
        }
    }

    fn visit_this_expr(&mut self, expr: &Rc<ThisExpression>) -> Result<EnvironmentValue, Error> {
        let add = get_rc_ref_address(expr.clone());
        let distance = self.scope_record.borrow();
        let distance = distance.get(&add).unwrap();
        let environment = Environment::get_env_by_distance(self.environment.clone(), *distance);
        let value = environment.borrow().get(&expr.keyword);
        value
    }

    fn visit_super_expr(&mut self, expr: &Rc<SuperExpression>) -> Result<EnvironmentValue, Error> {
        let add = get_rc_ref_address(expr.clone());
        let distance = self.scope_record.borrow();
        let distance = distance.get(&add).unwrap();
        let environment = Environment::get_env_by_distance(self.environment.clone(), *distance);

        let superclass = environment.borrow().get(&expr.keyword)?;

        match superclass {
            EnvironmentValue::LoxClass(superclass) => {
                let obj = Environment::get_env_by_distance(self.environment.clone(), *distance - 1);
                let obj = obj.borrow();
                let obj = obj.values.get(&THIS_STRING.as_ptr()).unwrap();

                let method = superclass.borrow().find_method(&expr.method.lexeme);

                if let Some(method) = method {
                    return Ok(method.clone().borrow_mut().bind(obj.clone()));
                }
            }
            _ => {}
        }
        Err(Error {
            line: expr.keyword.line,
            column: expr.keyword.column,
            message: format!("Undefined property {}", expr.method.lexeme),
        })
    }
}
