mod environment;
mod interpreter;
mod parser;
mod scanner;
mod semantic;
mod utils;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Abc {
    pub a: String,
}

fn main() {
    let now = std::time::Instant::now();
    let str = String::from(
        "
        var sum = 1;
        for( var i = 0; i < 10000000; i = i + 1){
          sum = sum + 1;
        }
        print sum;
    ",
    );
    let mut s = scanner::scanner::Scanner::new(&str);
    s.scan();

    let mut p = parser::parser::Parser::new(s.tokens.clone());
    p.parse();

    let statements = Rc::new(p.statements);

    let mut s_a = semantic::scope_analyst::ScopeAnalyst::new(statements.clone());
    s_a.analysis();

    let mut inter =
        interpreter::interpreter::Interpreter::new(statements.clone(), s_a.scope_record.clone());
    inter.interpret();

    println!("耗时:{:?}", now.elapsed());
}
