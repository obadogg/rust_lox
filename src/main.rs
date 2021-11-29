mod environment;
mod interpreter;
mod parser;
mod scanner;
mod semantic;
mod utils;
use std::rc::Rc;

use std::time::Instant;

#[derive(Debug, Clone)]
struct Abc {
    pub a: String,
}

fn main() {
    let now = Instant::now();
    let str = String::from(
        "
        class Person {
            init(name1, birth1) {
              this.name = name1;
              this.birth = birth1;
            }
          
            introduceMySelf() {
              print \"my name is \" + this.name;
              print \"i am \"  + \" years old\";
              print \"thanks for coming\";
              return 1111;
            }
          }
          
          var me = Person(\"aadonkeyz\", 1995);
          print me.introduceMySelf();

          var sum = 1;
          for(var i = 0;i < 10000000; i = i + 1){
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
    // println!("{:#?} , {}", statements.clone(), p.expr_count);

    let mut s_a = semantic::scope_analyst::ScopeAnalyst::new(statements.clone());
    s_a.analysis();

    let mut inter =
        interpreter::interpreter::Interpreter::new(statements.clone(), s_a.scope_record.clone());

    inter.interpret();

    let dur = now.elapsed();

    println!("耗时: {:?}", dur);
}
