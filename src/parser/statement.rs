use super::super::scanner::tokens::*;
use super::expression::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(ExpressionStatement),
    If(Rc<IfStatement>),
    Print(PrintStatement),
    While(Rc<WhileStatement>),
    For(Rc<ForStatement>),
    Var(VarStatement),
    Block(Rc<BlockStatement>),
    Function(Rc<FunctionStatement>),
    Return(ReturnStatement),
    Class(Rc<ClassStatement>),
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expr,
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub keyword: Token,
    pub expression: Expr,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Stmt,
}

#[derive(Debug)]
pub struct ForStatement {
    pub initializer: Option<Stmt>,
    pub condition: Option<Expr>,
    pub updator: Option<Expr>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct VarStatement {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Expr>,
    pub methods: Vec<Rc<FunctionStatement>>,
}
