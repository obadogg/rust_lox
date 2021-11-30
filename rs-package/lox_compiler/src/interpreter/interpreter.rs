use crate::environment::{environment::*, environment_value::*};
use crate::parser::{expression::*, statement::*};
use crate::scanner::{scanner::Error, tokens::*};
use crate::semantic::scope_analyst::*;
use crate::utils::utils::get_rc_ref_address;

use super::lox_class::*;
use super::lox_function::LoxFunction;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub envs: EnvironmentList,
    statements: Rc<Vec<Stmt>>,
    scope_record: Rc<RefCell<BTreeMap<usize, usize>>>,
    pub return_val: EnvironmentValue,
    log_fn: Option<fn(String) -> ()>,
}

impl Interpreter {
    pub fn new(
        statements: Rc<Vec<Stmt>>,
        scope_record: Rc<RefCell<BTreeMap<usize, usize>>>,
        log_fn: Option<fn(String) -> ()>,
    ) -> Self {
        Interpreter {
            envs: EnvironmentList::new(),
            statements,
            scope_record,
            return_val: EnvironmentValue::None,
            log_fn,
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
        let lox_function = LoxFunction::new(stmt.clone(), self.envs.env_pos, false);
        self.envs.define(
            ScopeAnalyst::get_scope_key_name(&stmt.name.lexeme),
            EnvironmentValue::LoxFunction(Rc::new(RefCell::new(lox_function))),
        )?;
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
            if self.log_fn.is_none() {
                println!("{}", message);
            } else {
                self.log_fn.unwrap()(message);
            }
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
        self.envs.next(None);

        if let Some(initializer) = &stmt.initializer {
            self.evaluate_statement_item(initializer)?;
        }

        if let Some(condition) = &stmt.condition {
            let mut flag = self.evaluate_expression_item(condition)?.is_truthy();
            while flag {
                self.evaluate_statement_item(&stmt.body)?;

                if let Some(updator) = &stmt.updator {
                    self.evaluate_expression_item(updator)?;
                    flag = self.evaluate_expression_item(condition)?.is_truthy();
                }
            }
        }
        self.envs.back();
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStatement) -> Result<(), Error> {
        let mut value = EnvironmentValue::None;
        if let Some(initializer) = &stmt.initializer {
            value = self.evaluate_expression_item(initializer)?;
        }
        self.envs.define(stmt.name.lexeme.as_ptr(), value)?;
        Ok(())
    }

    pub fn visit_block_stmt(
        &mut self,
        stmt: &BlockStatement,
        environment: Option<usize>,
    ) -> Result<(), Error> {
        let previous_env_pos = self.envs.env_pos;

        if let Some(env_pos) = environment {
            self.envs.go_to_env_by_pos(env_pos);
        } else {
            self.envs.next(None);
        }

        let stmts = &stmt.statements;

        for statement in stmts.iter() {
            if let Err(err) = self.evaluate_statement_item(statement) {
                self.envs.back();
                return Err(err);
            }
        }

        if environment.is_none() {
            self.envs.back();
        } else {
            self.envs.go_to_env_by_pos(previous_env_pos);
        }
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> Result<(), Error> {
        self.return_val = self.evaluate_expression_item(&stmt.value)?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &Rc<ClassStatement>) -> Result<(), Error> {
        self.envs
            .define(stmt.name.lexeme.as_ptr(), EnvironmentValue::None)?;

        let mut has_previous = false;
        let mut super_class = None;

        if let Some(superclass) = &stmt.superclass {
            let superclass_value = self.evaluate_expression_item(superclass)?;

            match superclass_value {
                EnvironmentValue::LoxClass(superclass_value_lox_class) => {
                    has_previous = true;
                    self.envs.next(None);
                    self.envs.define(
                        SUPER_STRING.as_ptr(),
                        EnvironmentValue::LoxClass(superclass_value_lox_class.clone()),
                    )?;
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

        let methods = stmt
            .methods
            .iter()
            .map(|f_stmt| {
                let is_init = *f_stmt.name.lexeme == INIT_STRING;
                let method = Rc::new(RefCell::new(LoxFunction::new(
                    f_stmt.clone(),
                    self.envs.env_pos,
                    is_init,
                )));

                if is_init {
                    (INIT_STRING.as_ptr(), method)
                } else {
                    (f_stmt.name.lexeme.as_ptr(), method)
                }
            })
            .collect::<BTreeMap<_, _>>();

        let lox_class = EnvironmentValue::LoxClass(Rc::new(RefCell::new(LoxClass::new(
            stmt.name.lexeme.clone(),
            super_class,
            methods,
        ))));

        if has_previous {
            self.envs.back();
        }

        self.envs.assign(&stmt.name, lox_class)?;
        Ok(())
    }

    fn visit_binary_expr(
        &mut self,
        expr: &Rc<BinaryExpression>,
    ) -> Result<EnvironmentValue, Error> {
        let left = self.evaluate_expression_item(&expr.left)?;
        let right = self.evaluate_expression_item(&expr.right)?;

        match expr.operator.token_type {
            TokensType::Plus => {
                if let Ok(value) = EnvironmentValue::add(&left, &right) {
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
                let result = EnvironmentValue::partial_eq(&left, &right);
                if result.is_ok() {
                    return Ok(result.unwrap());
                }
                Err(Error {
                    line: expr.operator.line,
                    column: expr.operator.column,
                    message: format!(
                        "\"!=\" and \"==\" operands only support number/string/boolean {}",
                        &expr.operator.lexeme
                    ),
                })
            }
            TokensType::EqualEqual => {
                let result = EnvironmentValue::eq(&left, &right);
                if result.is_ok() {
                    return Ok(result.unwrap());
                }
                Err(Error {
                    line: expr.operator.line,
                    column: expr.operator.column,
                    message: format!(
                        "\"!=\" and \"==\" operands only support number/string/boolean {}",
                        &expr.operator.lexeme
                    ),
                })
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
        let left_is_number = left.is_number();
        let right_is_number = right.is_number();

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
            TokensType::Minus => Ok(EnvironmentValue::sub(&left, &right).unwrap()),
            TokensType::Slash => Ok(EnvironmentValue::div(&left, &right).unwrap()),
            TokensType::Star => Ok(EnvironmentValue::mul(&left, &right).unwrap()),
            TokensType::Greater => Ok(EnvironmentValue::gt(&left, &right).unwrap()),
            TokensType::GreaterEqual => Ok(EnvironmentValue::ge(&left, &right).unwrap()),
            TokensType::Less => Ok(EnvironmentValue::lt(&left, &right).unwrap()),
            TokensType::LessEqual => Ok(EnvironmentValue::le(&left, &right).unwrap()),
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
                let right_is_number = right.is_number();
                if !right_is_number {
                    return Err(Error {
                        line: expr.operator.line,
                        column: expr.operator.column,
                        message: format!("Operand must be a number at {}", &expr.operator.lexeme),
                    });
                }
                return Ok(EnvironmentValue::neg(&right).unwrap());
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
            return Ok(self.envs.global_get(&expr.name)?.clone());
        }

        let borrow_env = self.scope_record.borrow();
        let distance = borrow_env.get(&add).unwrap();
        let value = Ok(self.envs.get_by_distance(&expr.name, *distance)?.clone());
        value
    }

    fn visit_assignment_expr(
        &mut self,
        expr: &Rc<AssignmentExpression>,
    ) -> Result<EnvironmentValue, Error> {
        let value = self.evaluate_expression_item(&expr.value)?;
        let add = get_rc_ref_address(expr.clone());

        if !self.scope_record.borrow().contains_key(&add) {
            self.envs.global_assign(&expr.name, value.clone())?;
            return Ok(value);
        }

        let borrow_env = self.scope_record.borrow();
        let distance = borrow_env.get(&add).unwrap();
        self.envs
            .assign_by_distance(&expr.name, *distance, value.clone())?;
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
                return lox_instance.get(&expr.name, self);
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
        let value = Ok(self.envs.get_by_distance(&expr.keyword, *distance)?.clone());
        value
    }

    fn visit_super_expr(&mut self, expr: &Rc<SuperExpression>) -> Result<EnvironmentValue, Error> {
        let add = get_rc_ref_address(expr.clone());
        let superclass;
        let distance;
        {
            let borrow_scope_record = self.scope_record.borrow();
            distance = borrow_scope_record.get(&add).unwrap().clone();
            superclass = self.envs.get_by_distance(&expr.keyword, distance)?.clone();
        }

        match superclass {
            EnvironmentValue::LoxClass(superclass) => {
                let obj = self
                    .envs
                    .get_by_distance_default(&THIS_STRING.as_ptr(), distance)
                    .unwrap()
                    .clone();

                let method = superclass
                    .borrow()
                    .find_method(&expr.method.lexeme.as_ptr());

                if let Some(method) = method {
                    return Ok(method.clone().borrow_mut().bind(obj.clone(), self)?);
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

    // fn visit_expr_wrap(&mut self,result:Result<EnvironmentValue, Error>) -> Result<EnvironmentValue, Error>{

    // }
}
