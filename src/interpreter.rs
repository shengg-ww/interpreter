// src/interpreter.rs
use crate::lexer::*;
use crate::parser::*;
use std::collections::HashMap;  // For HashMap


#[derive(Clone)]
pub struct RispEnv {
  data: HashMap<String, FlareLangExp>,
}



pub fn default_env() -> RispEnv {
    let mut data: HashMap<String, FlareLangExp> = HashMap::new();
    data.insert(
      "+".to_string(), 
      FlareLangExp::Func(
        |args: &[FlareLangExp]| -> Result<FlareLangExp, FlareLangErr> {
          let sum = parse_list_of_floats(args)?.iter().fold(0.0, |sum, a| sum + a);
          
          Ok(FlareLangExp::Number(sum))
        }
      )
    );
    data.insert(
      "-".to_string(), 
      FlareLangExp::Func(
        |args: &[FlareLangExp]| -> Result<FlareLangExp, FlareLangErr> {
          let floats = parse_list_of_floats(args)?;
          let first = *floats.first().ok_or(FlareLangErr::Reason("expected at least one number".to_string()))?;
          let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
          
          Ok(FlareLangExp::Number(first - sum_of_rest))
        }
      )
    );
    
    RispEnv {data}
  }




pub fn eval(exp: &FlareLangExp, env: &mut RispEnv) -> Result<FlareLangExp, FlareLangErr> {
    match exp {
      FlareLangExp::Symbol(k) =>
          env.data.get(k)
          .ok_or(
            FlareLangErr::Reason(
              format!("unexpected symbol k='{}'", k)
            )
          )
          .map(|x| x.clone())
      ,
     // If it's a number, return it
     FlareLangExp::Number(_) => Ok(exp.clone()),

     // If it's a string, return it
     FlareLangExp::String(_) => Ok(exp.clone()),

       // If it's a Print expression, evaluate the inside expression and print it
    FlareLangExp::Print(expr) => {
        let value = eval(expr, env)?;  // Evaluate the expression inside print
        match value {
            FlareLangExp::String(s) => {
                println!("{}", s);  // Print the string
            }
            FlareLangExp::Number(n) => {
                println!("{}", n);  // Print the number
            }
            _ => {}  // You can handle other types if needed
        }
        Ok(FlareLangExp::Number(0.0))  // Return a default value after printing
    },



      FlareLangExp::List(list) => {
        let first_form = list
          .first()
          .ok_or(FlareLangErr::Reason("expected a non-empty list".to_string()))?;
        let arg_forms = &list[1..];
        let first_eval = eval(first_form, env)?;
        match first_eval {
          FlareLangExp::Func(f) => {
            let args_eval = arg_forms
              .iter()
              .map(|x| eval(x, env))
              .collect::<Result<Vec<FlareLangExp>, FlareLangErr>>();
            f(&args_eval?)
          },
          _ => Err(
            FlareLangErr::Reason("first form must be a function".to_string())
          ),
        }
      },
      FlareLangExp::Func(_) => Err(
        FlareLangErr::Reason("unexpected form".to_string())
      ),
      FlareLangExp::Let(var_name, value_expr) => {
        let value = eval(value_expr, env)?;  // Evaluate the value of the expression
        env.data.insert(var_name.clone(), value);  // Bind the value to the variable
        Ok(FlareLangExp::Number(0.0))  // Return a default value (or you could return the value)
    },
     // Handle assignments (variable update)
     FlareLangExp::Assign(var_name, value_expr) => {
        let value = eval(value_expr, env)?;  // Evaluate the new value
        if env.data.contains_key(var_name) {
            env.data.insert(var_name.clone(), value.clone()) ;  // Update the value in the environment
            Ok(value)
        } else {
            Err(FlareLangErr::Reason(format!("undefined variable for assignment: {}", var_name)))
        }
    },

    // Handle arithmetic operations like `+`, `-`, `*`, `/`
    FlareLangExp::Op(op, left, right) => {
        let left_val = eval(left, env)?;
        let right_val = eval(right, env)?;

        match (left_val, right_val) {
            (FlareLangExp::Number(l), FlareLangExp::Number(r)) => match op.as_str() {
                "+" => Ok(FlareLangExp::Number(l + r)),
                "-" => Ok(FlareLangExp::Number(l - r)),
                "*" => Ok(FlareLangExp::Number(l * r)),
                "/" => {
                    if r == 0.0 {
                        Err(FlareLangErr::Reason("division by zero".to_string()))
                    } else {
                        Ok(FlareLangExp::Number(l / r))
                    }
                },
                _ => Err(FlareLangErr::Reason("unknown operator".to_string())),
            },
            _ => Err(FlareLangErr::Reason("operands must be numbers".to_string())),
        }
    },

    // Handle `if` expressions (conditional evaluation)
    FlareLangExp::If(condition, true_expr, false_expr) => {
        let cond_val = eval(condition, env)?;
        match cond_val {
            FlareLangExp::Number(n) if n != 0.0 => eval(true_expr, env),  // If true, evaluate true branch
            _ => eval(false_expr, env),  // Otherwise, evaluate false branch
        }
    },

}
    }
  

