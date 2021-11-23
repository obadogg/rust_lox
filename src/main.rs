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
    let str = String::from(
        "
        class Person {
            init(name, birth) {
              this.name = name;
              this.birth = birth;
            }

            introduceMySelf() {
              print \"my name is \" + this.name + \":fuck\";
              print \"thanks for coming\";
              return this.aaa();
            }

            aaa(){
              return 222;
            }
          }

          var me = Person(\"aadonkeyz\", 1995);
          print me.introduceMySelf();
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

    // println!("token length: {}", s.tokens.len());
}
