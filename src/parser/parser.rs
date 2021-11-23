use super::super::scanner::{scanner::*, tokens::*};
use super::{expression::*, statement::*};
use std::collections::VecDeque;
use std::rc::Rc;

/**
 * program        → declaration* EOF
 * declaration    → classDecl | funDecl | varDecl | statement
 * classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}"
 * funDecl        → "fun" function
 * function       → IDENTIFIER "(" parameters? ")" block
 * parameters     → IDENTIFIER ( "," IDENTIFIER )*
 * varDecl        → "var" IDENTIFIER ( "=" expression )? ";"
 * statement      → exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block
 * exprStmt       → expression ";"
 * ifStmt         → "if" "(" expression ")" statement ( "else" statement )?
 * forStmt        → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement
 * printStmt      → "print" expression ";"
 * returnStmt     → "return" expression? ";"
 * whileStmt      → "while" "(" expression ")" statement
 * block          → "{" declaration* "}" ;
 *
 *
 *
 * expression     → assignment
 * assignment     → ( call "." )? IDENTIFIER "=" assignment | logicOr
 * logicOr        → logicAnd ("or" logicAnd)*
 * logicAnd       → equality ("and" equality)*
 * equality       → comparison ( ( "!=" | "==" ) comparison )*
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )*
 * term           → factor ( ( "-" | "+" ) factor )*
 * factor         → unary ( ( "/" | "*" ) unary )*
 * unary          → ( "!" | "-" ) unary | call
 * call           → primary ( "(" arguments? ")" | "." IDENTIFIER )*
 * arguments      → expression ( "," expression )*
 * primary        → NUMBER | STRING | "true" | "false" | "nil" | "this" | IDENTIFIER | "(" expression ")" | "super" "." IDENTIFIER
 */

/*
   item ——一个项（item），像一个函数，结构体，模块等。
   block ——一个块 （block）（即一个语句块或一个表达式，由花括号所包围）
   stmt —— 一个语句（statement）
   pat ——一个模式（pattern）
   expr —— 一个表达式（expression）
   ty ——一个类型（type）
   ident—— 一个标识符（indentfier）
   path —— 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
   meta —— 一个元数据项；位于#[...]和#![...]属性
   tt——一个词法树
   vis——一个可能为空的Visibility限定词
*/
macro_rules! clone_previous_token {
    ($k:expr) => {{
        let a = $k.previous();
        let a = a.clone();
        a
    }};
}

#[derive(Debug)]
enum FunType {
    Function,
    Method,
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: VecDeque<Token>,
    pub current: u8,
    pub statements: Vec<Stmt>,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_end() {
            match self.declaration() {
                Ok(stmt) => {
                    self.statements.push(stmt);
                }
                Err(_) => {
                    self.synchronize();
                    panic!("oops!parser panic: {:#?}", self.errors);
                }
            }
        }
    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        if self.match_token(TokensType::Fun) {
            return Ok(Stmt::Function(Rc::new(self.fun_decl(FunType::Function)?)));
        }
        if self.match_token(TokensType::Class) {
            return Ok(Stmt::Class(Rc::new(self.class_decl()?)));
        }
        if self.match_token(TokensType::Var) {
            return Ok(Stmt::Var(self.var_decl()?));
        }

        return Ok(self.statement()?);
    }

    fn fun_decl(&mut self, fun_type: FunType) -> Result<FunctionStatement, ()> {
        let name = self.consume(
            TokensType::Identifier,
            format!("Expect {:?} name", fun_type),
        )?;
        let name = name.clone();
        self.consume(
            TokensType::LeftParen,
            format!("Expect \"(\" after {:?} name", fun_type),
        )?;

        let mut params = Vec::new();
        if !self.check(TokensType::RightParen) {
            loop {
                let param = self.consume(
                    TokensType::Identifier,
                    String::from("Expect parameter name"),
                )?;
                params.push(param.clone());
                if !self.match_token(TokensType::Comma) {
                    break;
                }
            }
        }
        self.consume(
            TokensType::RightParen,
            String::from("Expect \")\" after parameters"),
        )?;
        self.consume(
            TokensType::LeftBrace,
            format!("Expect \"{{\" before {:?} body", fun_type),
        )?;

        let body = self.block()?;
        let body = BlockStatement { statements: body };
        Ok(FunctionStatement { name, params, body })
    }

    fn class_decl(&mut self) -> Result<ClassStatement, ()> {
        let name = self.consume(TokensType::Identifier, String::from("Expect class name"))?;
        let name = name.clone();

        let mut superclass = None;
        if self.match_token(TokensType::Less) {
            let superclass_name = self.previous();
            let superclass_name = superclass_name.clone();
            self.consume(
                TokensType::Identifier,
                String::from("Expect superclass name'"),
            )?;
            superclass = Some(Expr::Variable(Rc::new(VariableExpression {
                name: superclass_name,
            })));
        }
        self.consume(
            TokensType::LeftBrace,
            String::from("Expect \"{\" before class body"),
        )?;

        let mut methods = Vec::new();
        while !self.check(TokensType::RightBrace) && !self.is_end() {
            methods.push(Rc::new(self.fun_decl(FunType::Method)?));
        }

        self.consume(
            TokensType::RightBrace,
            String::from("Expect \"}\" after class body"),
        )?;

        Ok(ClassStatement {
            name,
            superclass,
            methods,
        })
    }

    fn var_decl(&mut self) -> Result<VarStatement, ()> {
        let name = self.consume(TokensType::Identifier, String::from("Expect variable name"))?;
        let name = name.clone();
        let mut initializer = None;

        if self.match_token(TokensType::Equal) {
            initializer = Some(self.expression()?);
        }
        self.consume(
            TokensType::Semicolon,
            String::from("Expect \";\" after variable declaration"),
        )?;
        Ok(VarStatement { name, initializer })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ()> {
        let mut statements = Vec::new();

        while !self.check(TokensType::RightBrace) && !self.is_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        self.consume(
            TokensType::RightBrace,
            String::from("Expect \"}\" after block"),
        )?;
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.match_token(TokensType::If) {
            return self.if_stmt();
        }

        if self.match_token(TokensType::Print) {
            return self.print_stmt();
        }

        if self.match_token(TokensType::Return) {
            return self.return_stmt();
        }

        if self.match_token(TokensType::While) {
            return self.while_stmt();
        }

        if self.match_token(TokensType::For) {
            return self.for_stmt();
        }

        if self.match_token(TokensType::LeftBrace) {
            return Ok(Stmt::Block(Rc::new(BlockStatement {
                statements: self.block()?,
            })));
        }

        self.expr_stmt()
    }

    fn if_stmt(&mut self) -> Result<Stmt, ()> {
        self.consume(
            TokensType::LeftParen,
            String::from("Expect \"(\" after \"if\""),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokensType::RightParen,
            String::from("Expect \")\" after if condition"),
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.match_token(TokensType::Else) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::If(Rc::new(IfStatement {
            condition,
            then_branch,
            else_branch,
        })))
    }

    fn print_stmt(&mut self) -> Result<Stmt, ()> {
        let keyword = clone_previous_token!(self);
        let expression = self.expression()?;
        self.consume(
            TokensType::Semicolon,
            String::from("Expect \";\" after value"),
        )?;
        Ok(Stmt::Print(PrintStatement {
            keyword,
            expression,
        }))
    }

    fn return_stmt(&mut self) -> Result<Stmt, ()> {
        let keyword = clone_previous_token!(self);
        let value;
        if self.check(TokensType::Semicolon) {
            value = Expr::Literal(LiteralExpression { value: None });
        } else {
            value = self.expression()?;
        }

        self.consume(
            TokensType::Semicolon,
            String::from("Expect \";\" after return value"),
        )?;

        Ok(Stmt::Return(ReturnStatement { keyword, value }))
    }

    fn while_stmt(&mut self) -> Result<Stmt, ()> {
        self.consume(
            TokensType::LeftParen,
            String::from("Expect \"(\" after \"while\""),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokensType::RightParen,
            String::from("Expect \")\" after condition"),
        )?;
        let body = self.statement()?;
        Ok(Stmt::While(Rc::new(WhileStatement { condition, body })))
    }

    fn for_stmt(&mut self) -> Result<Stmt, ()> {
        self.consume(
            TokensType::LeftParen,
            String::from("Expect \"(\" after \"for\""),
        )?;

        if self.check(TokensType::RightParen) {
            self.errors.push(Error {
                line: self.peek().line,
                column: self.peek().column,
                message: String::from("There is nothing exist in the parenthese of \"for\""),
            });
            return Err(());
        }

        let mut initializer = None;
        let mut condition = None;
        let mut updator = None;

        if !self.check(TokensType::Semicolon) {
            initializer = Some(self.declaration()?);
        } else {
            self.advance();
        }

        if !self.match_token(TokensType::Semicolon) {
            condition = Some(self.expression()?);
            self.consume(
                TokensType::Semicolon,
                String::from("Expect \";\" after the condition of \"for\""),
            )?;
        }

        if !self.match_token(TokensType::RightParen) {
            updator = Some(self.expression()?);
            self.consume(
                TokensType::RightParen,
                String::from("Expect \")\" after the parenthese of \"for\""),
            )?;
        }
        let body = self.statement()?;

        Ok(Stmt::For(Rc::new(ForStatement {
            initializer,
            condition,
            updator,
            body,
        })))
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ()> {
        let expression = self.expression()?;
        self.consume(
            TokensType::Semicolon,
            String::from("Expect \";\" after expression"),
        )?;
        Ok(Stmt::Expression(ExpressionStatement { expression }))
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expression = self.logic_or()?;

        if self.match_token(TokensType::Equal) {
            let equals = clone_previous_token!(self);
            let value = self.assignment()?;

            match expression {
                Expr::Variable(variable) => {
                    let name = variable.name.clone();
                    return Ok(Expr::Assignment(Rc::new(AssignmentExpression {
                        name,
                        value,
                    })));
                }
                Expr::Get(get_expression) => {
                    return Ok(Expr::Set(Rc::new(SetExpression {
                        object: get_expression.object.clone(),
                        name: get_expression.name.clone(),
                        value,
                    })))
                }
                _ => {}
            }

            self.errors.push(Error {
                line: equals.line,
                column: equals.column,
                message: String::from("Invalid assignment target"),
            });
            return Err(());
        }
        Ok(expression)
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        if self.match_token(TokensType::Number) || self.match_token(TokensType::String) {
            let literal = clone_previous_token!(self).literal;
            return Ok(Expr::Literal(LiteralExpression { value: literal }));
        }

        if self.match_token(TokensType::True) {
            return Ok(Expr::Literal(LiteralExpression {
                value: Some(ValueType::Bool(true)),
            }));
        }

        if self.match_token(TokensType::False) {
            return Ok(Expr::Literal(LiteralExpression {
                value: Some(ValueType::Bool(false)),
            }));
        }

        if self.match_token(TokensType::Nil) {
            return Ok(Expr::Literal(LiteralExpression { value: None }));
        }

        if self.match_token(TokensType::This) {
            let keyword = self.previous();
            let keyword = keyword.clone();
            return Ok(Expr::This(Rc::new(ThisExpression { keyword })));
        }

        if self.match_token(TokensType::Super) {
            let keyword = clone_previous_token!(self);
            self.consume(
                TokensType::Dot,
                String::from("Expect \".\" after \"super\""),
            )?;
            self.consume(
                TokensType::Identifier,
                String::from("Expect superclass method name"),
            )?;
            let method = clone_previous_token!(self);
            return Ok(Expr::Super(Rc::new(SuperExpression { keyword, method })));
        }

        if self.match_token(TokensType::Identifier) {
            let name = clone_previous_token!(self);
            return Ok(Expr::Variable(Rc::new(VariableExpression { name })));
        }

        if self.match_token(TokensType::LeftParen) {
            let expression = Ok(Expr::Grouping(Rc::new(GroupingExpression {
                expression: self.expression()?,
            })));
            self.consume(
                TokensType::RightParen,
                String::from("Expect \")\" after expression"),
            )?;
            return expression;
        }

        self.errors.push(Error {
            line: self.peek().line,
            column: self.peek().column,
            message: format!("Unexpected token \"{:?}\"", self.peek().lexeme),
        });

        Err(())
    }

    fn logic_or(&mut self) -> Result<Expr, ()> {
        let mut expression = self.logic_and()?;

        while self.match_token(TokensType::Or) {
            let operator = clone_previous_token!(self);
            let right = self.logic_and()?;
            expression = Expr::Logical(Rc::new(LogicalExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn logic_and(&mut self) -> Result<Expr, ()> {
        let mut expression = self.equality()?;

        while self.match_token(TokensType::And) {
            let operator = clone_previous_token!(self);
            let right = self.equality()?;
            expression = Expr::Logical(Rc::new(LogicalExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expression = self.comparison()?;

        while self.match_token(TokensType::BangEqual) || self.match_token(TokensType::EqualEqual) {
            let operator = clone_previous_token!(self);
            let right = self.comparison()?;
            expression = Expr::Binary(Rc::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expression = self.term()?;

        while self.match_token(TokensType::Greater)
            || self.match_token(TokensType::GreaterEqual)
            || self.match_token(TokensType::Less)
            || self.match_token(TokensType::LessEqual)
        {
            let operator = clone_previous_token!(self);
            let right = self.term()?;
            expression = Expr::Binary(Rc::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut expression = self.factor()?;

        while self.match_token(TokensType::Minus) || self.match_token(TokensType::Plus) {
            let operator = clone_previous_token!(self);
            let right = self.factor()?;

            expression = Expr::Binary(Rc::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expression = self.unary()?;

        while self.match_token(TokensType::Slash) || self.match_token(TokensType::Star) {
            let operator = clone_previous_token!(self);
            let right = self.unary()?;
            expression = Expr::Binary(Rc::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if self.match_token(TokensType::Bang) || self.match_token(TokensType::Minus) {
            let operator = clone_previous_token!(self);
            let right = self.unary()?;
            return Ok(Expr::Unary(Rc::new(UnaryExpression {
                operator,
                expression: right,
            })));
        }

        Ok(self.call()?)
    }

    fn call(&mut self) -> Result<Expr, ()> {
        let mut expression = self.primary()?;

        while self.match_token(TokensType::LeftParen) || self.match_token(TokensType::Dot) {
            let previous_type = self.previous().token_type;
            match previous_type {
                TokensType::LeftParen => expression = self.finish_call(expression)?,
                TokensType::Dot => {
                    let name = self.consume(
                        TokensType::Identifier,
                        String::from("Expect property name after \".\""),
                    )?;
                    let name = name.clone();
                    expression = Expr::Get(Rc::new(GetExpression {
                        object: expression,
                        name,
                    }))
                }
                _ => {}
            }
        }

        Ok(expression)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ()> {
        let mut args = Vec::new();

        loop {
            if !self.check(TokensType::RightParen) {
                args.push(self.expression()?);
            }
            if !self.match_token(TokensType::Comma) {
                break;
            }
        }

        let end_parenthese = self.consume(
            TokensType::RightParen,
            String::from("Expect \")\" after arguments"),
        )?;
        let end_parenthese = end_parenthese.clone();

        Ok(Expr::Call(Rc::new(CallExpression {
            callee,
            args,
            end_parenthese,
        })))
    }

    fn match_token(&mut self, token_type: TokensType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokensType) -> bool {
        if self.is_end() {
            return false;
        }
        return token_type == self.peek().token_type;
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokensType::Eof
    }

    fn peek(&self) -> &Token {
        // if self.current as usize >= self.tokens.len() {
        //     return None;
        // }
        &self.tokens[self.current as usize]
    }

    fn previous(&self) -> &Token {
        let index = (self.current - 1) as usize;
        // if index >= self.tokens.len() {
        //     return None;
        // }
        &self.tokens[index]
    }

    fn consume(&mut self, token_type: TokensType, message: String) -> Result<&Token, ()> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        self.errors.push(Error {
            line: self.previous().line,
            column: self.previous().column,
            message,
        });
        Err(())
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if self.previous().token_type == TokensType::Semicolon {
                return ();
            }

            let t = self.peek().token_type;

            match t {
                TokensType::Class
                | TokensType::Fun
                | TokensType::Var
                | TokensType::For
                | TokensType::If
                | TokensType::While
                | TokensType::Print
                | TokensType::Return => return (),
                _ => {}
            }

            self.advance();
        }
    }
}
