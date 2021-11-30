mod environment;
mod interpreter;
pub mod parser;
pub mod scanner;
mod semantic;
mod utils;

use std::{collections::VecDeque, rc::Rc};

pub fn parse_token(code: &String) -> VecDeque<Rc<scanner::tokens::Token>> {
    let mut s = scanner::scanner::Scanner::new(code);
    s.scan();
    s.tokens
}

pub fn parse(code: &String) -> Rc<Vec<parser::statement::Stmt>> {
    let tokens = parse_token(code);
    let mut p = parser::parser::Parser::new(tokens);
    p.parse();
    Rc::new(p.statements)
}

pub fn interpret(code: &String) {
    let statements = parse(code);

    let mut s_a = semantic::scope_analyst::ScopeAnalyst::new(statements.clone());
    s_a.analysis();

    let mut inter =
        interpreter::interpreter::Interpreter::new(statements.clone(), s_a.scope_record.clone());

    inter.interpret();
}
