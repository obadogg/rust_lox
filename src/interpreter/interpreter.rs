use crate::environment::environment::*;
use crate::parser::{expression::*, statement::*};
use crate::scanner::scanner::Error;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::lox_function::LoxFunction;

#[derive(Debug, Clone)]
pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    statements: Rc<Vec<Stmt>>,
    scope_record: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn new(statements: Rc<Vec<Stmt>>, scope_record: HashMap<usize, usize>) -> Self {
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
            // Expr::Binary(expr_binary) => self.visit_binary_expr(expr_binary),
            // Expr::Logical(expr_logical) => self.visit_logical_expr(expr_logical),
            // Expr::Grouping(expr_grouping) => self.visit_grouping_expr(expr_grouping),
            // Expr::Literal(_) => (),
            // Expr::Unary(expr_unary) => self.visit_unary_expr(expr_unary),
            // Expr::Variable(expr_variable) => self.visit_variable_expr(expr_variable),
            // Expr::Assignment(expr_assignment) => self.visit_assignment_expr(expr_assignment),
            // Expr::Call(expr_call) => self.visit_call_expr(expr_call),
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
            Stmt::Block(stmt_block) => self.visit_block_stmt(stmt_block),
            Stmt::Return(stmt_return) => self.visit_return_stmt(stmt_return),
            Stmt::Class(stmt_class) => self.visit_class_stmt(stmt_class),
            _ => Ok(()),
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Rc<FunctionStatement>) -> Result<(), Error> {
        let lox_function = LoxFunction::new(stmt.clone(), self.environment.clone(), false);
        self.environment.borrow_mut().define(
            stmt.name.lexeme.clone(),
            Some(EnvironmentValue::LoxFunction(lox_function)),
        );
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> Result<(), Error> {
        self.evaluate_expression_item(&stmt.condition)?;

        self.evaluate_statement_item(&stmt.then_branch);

        if let Some(else_branch) = &stmt.else_branch {
            self.evaluate_statement_item(else_branch);
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
        let mut value: Option<EnvironmentValue> = None;
        if let Some(initializer) = &stmt.initializer {
            value = Some(self.evaluate_expression_item(initializer)?);
        }
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStatement) -> Result<(), Error> {
        let stmts = &stmt.statements;

        for statement in stmts.iter() {
            self.evaluate_statement_item(statement)?;
        }
        //TODO:
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
            .define(stmt.name.lexeme.clone(), None);

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
}
