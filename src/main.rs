
use std::fmt;                    // For fmt::Display
use std::io::{self}; 
mod lexer;
mod parser;
mod interpreter;
use crate::lexer::*;
use crate::parser::*;
use crate::interpreter::*;



impl fmt::Display for FlareLangExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let str = match self {
        FlareLangExp::Symbol(s) => s.clone(),
        FlareLangExp::Number(n) => n.to_string(),
        FlareLangExp::String(s) => s.to_string(),
        FlareLangExp::List(list) => {
          let xs: Vec<String> = list
            .iter()
            .map(|x| x.to_string())
            .collect();
          format!("({})", xs.join(","))
        },
        FlareLangExp:: Print(expr) => format!("display {}", expr.to_string()),
        FlareLangExp::Func(_) => "Function {}".to_string(),
        FlareLangExp::Let(var, expr) => format!("let {} = {}", var, expr.to_string()),  // For `let` bindings, display `let <var> = <expr>`
        FlareLangExp::Assign(var, expr) => format!("{} = {}", var, expr.to_string()),  // For assignments, display `<var> = <expr>`
        FlareLangExp::Op(op, left, right) => {
            format!("{} {} {}", left.to_string(), op, right.to_string())  // For operations, display `left op right`
        },
        FlareLangExp::If(cond, true_expr, false_expr) => {
            format!("if {} {{ {} }} else {{ {} }}", cond.to_string(), true_expr.to_string(), false_expr.to_string())  // For `if` expressions, display `if <cond> { <true_expr> } else { <false_expr> }`
        },
      };
      
      write!(f, "{}", str)
    }
  }


  fn parse_eval(expr: String, env: &mut RispEnv) -> Result<FlareLangExp, FlareLangErr> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    
    Ok(evaled_exp)
  }
  
  fn slurp_expr() -> String {
    let mut expr = String::new();
    
    io::stdin().read_line(&mut expr)
      .expect("Failed to read line");
    
    expr
  }
  fn main() {

    let env = &mut default_env();
    loop {
      println!("FlareLang >");
      let expr = slurp_expr();
      match parse_eval(expr, env) {
        Ok(res) => println!("// SUCCESS => {}", res),
        Err(e) => match e {
          FlareLangErr::Reason(msg) => println!("// ERROR => {}", msg),
        },
      }
    }
  }