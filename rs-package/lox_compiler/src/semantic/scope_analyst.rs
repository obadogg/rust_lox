use crate::map;
use crate::utils::utils::get_rc_ref_address;

use super::super::parser::{expression::*, statement::*};
use super::super::scanner::{scanner::*, tokens::*};

use std::collections::BTreeMap;

use std::{cell::RefCell, rc::Rc};

pub static THIS_STRING: &'static str = "this";
pub static SUPER_STRING: &'static str = "super";
pub static INIT_STRING: &'static str = "init";

#[derive(Debug, Copy, Clone)]
pub enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
}

#[derive(Debug, Copy, Clone)]
pub enum ClassType {
    None,
    Class,
    SubClass,
}

#[derive(Debug, Clone)]
pub struct ScopeAnalyst {
    pub statements: Rc<Vec<Stmt>>,
    pub scopes: Vec<BTreeMap<*const u8, bool>>,
    pub scope_record: Rc<RefCell<BTreeMap<usize, usize>>>,
    pub function_type: FunctionType,
    pub class_type: ClassType,
    pub errors: Vec<Error>,
}

impl ScopeAnalyst {
    pub fn new(statements: Rc<Vec<Stmt>>) -> Self {
        ScopeAnalyst {
            statements,
            scopes: Vec::new(),
            scope_record: Rc::new(RefCell::new(BTreeMap::new())),
            function_type: FunctionType::None,
            class_type: ClassType::None,
            errors: Vec::new(),
        }
    }

    pub fn analysis(&mut self) {
        self.evaluate_statement_list(&self.statements.clone());
    }

    fn evaluate_statement_list(&mut self, stmts: &Vec<Stmt>) {
        for statement in stmts.iter() {
            self.evaluate_statement_item(statement);
        }
    }

    fn evaluate_statement_item(&mut self, stmt: &Stmt) {
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
        }
    }

    fn evaluate_expression_list(&mut self, exprs: &Vec<Expr>) {
        for expr in exprs.iter() {
            self.evaluate_expression_item(expr);
        }
    }

    fn evaluate_expression_item(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary(expr_binary) => self.visit_binary_expr(expr_binary),
            Expr::Logical(expr_logical) => self.visit_logical_expr(expr_logical),
            Expr::Grouping(expr_grouping) => self.visit_grouping_expr(expr_grouping),
            Expr::Literal(_) => (),
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

    fn visit_function_stmt(&mut self, stmt: &FunctionStatement) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.evaluate_function(stmt, FunctionType::Function);
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) {
        self.evaluate_expression_item(&stmt.condition);
        self.evaluate_statement_item(&stmt.then_branch);

        if let Some(else_branch) = &stmt.else_branch {
            self.evaluate_statement_item(else_branch);
        }
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStatement) {
        self.evaluate_expression_item(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStatement) {
        self.evaluate_expression_item(&stmt.expression);
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStatement) {
        self.evaluate_expression_item(&stmt.condition);
        self.evaluate_statement_item(&stmt.body);
    }

    fn visit_for_stmt(&mut self, stmt: &ForStatement) {
        self.scopes.push(BTreeMap::new());
        if let Some(initializer) = &stmt.initializer {
            self.evaluate_statement_item(initializer);
        }
        if let Some(condition) = &stmt.condition {
            self.evaluate_expression_item(condition);
        }
        if let Some(updator) = &stmt.updator {
            self.evaluate_expression_item(updator);
        }
        self.evaluate_statement_item(&stmt.body);
        self.scopes.pop();
    }

    fn visit_var_stmt(&mut self, stmt: &VarStatement) {
        self.declare(&stmt.name);
        if let Some(initializer) = &stmt.initializer {
            self.evaluate_expression_item(initializer);
        }
        self.define(&stmt.name);
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStatement) {
        self.scopes.push(BTreeMap::new());
        self.evaluate_statement_list(&stmt.statements);
        self.scopes.pop();
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) {
        match self.function_type {
            FunctionType::None => {
                self.errors.push(Error {
                    line: stmt.keyword.line,
                    column: stmt.keyword.column,
                    message: String::from("Can't return from top-level code"),
                });
            }
            _ => {}
        }

        let mut closure = || {
            match self.function_type {
                FunctionType::Initializer => {
                    self.errors.push(Error {
                        line: stmt.keyword.line,
                        column: stmt.keyword.column,
                        message: String::from("Can't use return a value from an initializer"),
                    });
                }
                _ => {}
            }
            self.evaluate_expression_item(&stmt.value);
        };

        match &stmt.value {
            Expr::Literal(literal_expr) => {
                if let Some(_) = &literal_expr.value {
                    closure();
                }
            }
            _ => {
                closure();
            }
        }

        self.evaluate_expression_item(&stmt.value);
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStatement) {
        let previous_class_type = self.class_type;
        self.class_type = ClassType::Class;
        self.declare(&stmt.name);

        if let Some(superclass) = &stmt.superclass {
            match superclass {
                Expr::Variable(superclass_expr) => {
                    if stmt.name.lexeme == superclass_expr.name.lexeme {
                        self.errors.push(Error {
                            line: stmt.name.line,
                            column: stmt.name.column,
                            message: format!(
                                r#"A class can't inherit from itself("{:?}""#,
                                stmt.name.lexeme
                            ),
                        })
                    }
                }
                _ => {}
            }
            self.evaluate_expression_item(superclass);
            self.scopes.push(map! {SUPER_STRING.as_ptr() => true});
            self.class_type = ClassType::SubClass;
        }

        self.scopes.push(map! {THIS_STRING.as_ptr() => true});

        for method in stmt.methods.iter() {
            if *method.name.lexeme == "init" {
                self.evaluate_function(method, FunctionType::Initializer);
            } else {
                self.evaluate_function(method, FunctionType::Method)
            }
        }
        self.scopes.pop();

        if let Some(_) = stmt.superclass {
            self.scopes.pop();
        }

        self.define(&stmt.name);
        self.class_type = previous_class_type;
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpression) {
        self.evaluate_expression_item(&expr.left);
        self.evaluate_expression_item(&expr.right);
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpression) {
        self.evaluate_expression_item(&expr.left);
        self.evaluate_expression_item(&expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpression) {
        self.evaluate_expression_item(&expr.expression);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpression) {
        self.evaluate_expression_item(&expr.expression);
    }

    fn visit_variable_expr(&mut self, expr: &Rc<VariableExpression>) {
        if self.scopes.len() != 0 {
            let last = self.scopes.last().unwrap();
            if let Some(flag) = last.get(&ScopeAnalyst::get_scope_key_name(&expr.name.lexeme)) {
                if !flag {
                    self.errors.push(Error {
                        line: expr.name.line,
                        column: expr.name.column,
                        message: format!(
                            r#"Can't read local variable in its own initializer("{}")"#,
                            expr.name.lexeme
                        ),
                    });
                }
            }
        }

        let add = get_rc_ref_address(expr.clone());
        self.calculate(add, &expr.name);
    }

    fn visit_assignment_expr(&mut self, expr: &Rc<AssignmentExpression>) {
        self.evaluate_expression_item(&expr.value);

        let add = get_rc_ref_address(expr.clone());
        self.calculate(add, &expr.name);
    }

    fn visit_call_expr(&mut self, expr: &CallExpression) {
        self.evaluate_expression_item(&expr.callee);
        self.evaluate_expression_list(&expr.args);
    }

    fn visit_get_expr(&mut self, expr: &GetExpression) {
        self.evaluate_expression_item(&expr.object);
    }

    fn visit_set_expr(&mut self, expr: &SetExpression) {
        self.evaluate_expression_item(&expr.value);
        self.evaluate_expression_item(&expr.object);
    }

    fn visit_this_expr(&mut self, expr: &Rc<ThisExpression>) {
        match self.class_type {
            ClassType::None => self.errors.push(Error {
                line: expr.keyword.line,
                column: expr.keyword.column,
                message: String::from(r#"Can't use "this" outside of a class"#),
            }),
            _ => {}
        }

        let add = get_rc_ref_address(expr.clone());
        self.calculate(add, &expr.keyword);
    }

    fn visit_super_expr(&mut self, expr: &Rc<SuperExpression>) {
        match self.class_type {
            ClassType::None => self.errors.push(Error {
                line: expr.keyword.line,
                column: expr.keyword.column,
                message: String::from(r#"Can't use "super" outside of a class"#),
            }),
            ClassType::Class => self.errors.push(Error {
                line: expr.keyword.line,
                column: expr.keyword.column,
                message: String::from(r#"Can't use "super" in a class with no superclass"#),
            }),
            _ => {}
        }

        let add = get_rc_ref_address(expr.clone());
        self.calculate(add, &expr.keyword);
    }

    fn calculate(&mut self, address: usize, token: &Token) {
        for (pos, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&ScopeAnalyst::get_scope_key_name(&token.lexeme)) {
                self.scope_record.borrow_mut().insert(address, pos);
            }
        }
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.len() != 0 {
            let last = self.scopes.last_mut().unwrap();
            last.insert(ScopeAnalyst::get_scope_key_name(&name.lexeme), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.len() != 0 {
            let last = self.scopes.last_mut().unwrap();
            last.insert(ScopeAnalyst::get_scope_key_name(&name.lexeme), true);
        }
    }

    fn evaluate_function(&mut self, stmt: &FunctionStatement, function_type: FunctionType) {
        let previous_function_type = self.function_type;
        self.function_type = function_type;

        self.scopes.push(BTreeMap::new());

        for statement in &stmt.params {
            self.declare(statement);
            self.define(statement);
        }
        self.evaluate_statement_list(&stmt.body.statements);

        self.scopes.pop();

        self.function_type = previous_function_type;
    }

    pub fn get_scope_key_name(lexeme: &Rc<String>) -> *const u8 {
        if **lexeme == INIT_STRING {
            return INIT_STRING.as_ptr();
        }

        if **lexeme == THIS_STRING {
            return THIS_STRING.as_ptr();
        }

        if **lexeme == SUPER_STRING {
            return SUPER_STRING.as_ptr();
        }
        return lexeme.as_ptr();
    }
}
