use super::super::scanner::tokens::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Rc<BinaryExpression>),
    Logical(Rc<LogicalExpression>),
    Grouping(Rc<GroupingExpression>),
    Literal(LiteralExpression),
    Unary(Rc<UnaryExpression>),
    Variable(Rc<VariableExpression>),
    Assignment(Rc<AssignmentExpression>),
    Call(Rc<CallExpression>),
    Get(Rc<GetExpression>),
    Set(Rc<SetExpression>),
    This(Rc<ThisExpression>),
    Super(Rc<SuperExpression>),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug)]
pub struct LogicalExpression {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug)]
pub struct GroupingExpression {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub value: Option<ValueType>,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub name: Token,
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub name: Token,
    pub value: Expr,
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Expr,
    pub args: Vec<Expr>,
    pub end_parenthese: Token,
}

#[derive(Debug)]
pub struct GetExpression {
    pub object: Expr,
    pub name: Token,
}

#[derive(Debug)]
pub struct SetExpression {
    pub object: Expr,
    pub name: Token,
    pub value: Expr,
}

#[derive(Debug)]
pub struct ThisExpression {
    pub keyword: Token,
}

#[derive(Debug, Clone)]
pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}
