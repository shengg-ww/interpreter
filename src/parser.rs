use crate::lexer::{FlareLangExp,FlareLangErr};

pub fn read_seq<'a>(tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    let mut res: Vec<FlareLangExp> = vec![];  // Holds the parsed expressions inside the parentheses
    let mut xs = tokens;  // Start with the full token list

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(FlareLangErr::Reason("could not find closing `)`".to_string()))?;  // If no tokens, error out
        
        if next_token == ")" {
            return Ok((FlareLangExp::List(res), rest));  // Return the parsed list once we hit `)`
        }

        // Parse the next expression and continue
        let (exp, new_xs) = parse(xs)?;  // Pass xs directly
        res.push(exp);
        xs = new_xs;  // Update xs to the remaining tokens
    }
}



pub fn parse<'a>(tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    let (token, rest) = tokens.split_first()
        .ok_or(FlareLangErr::Reason("could not get token".to_string()))?;

    match &token[..] {
        // Handle string literals
        token if token.starts_with("\"") && token.ends_with("\"") => {
            let string_value = token[1..token.len()-1].to_string();  // Remove the quotes
            Ok((FlareLangExp::String(string_value), rest))
        },

        // Handle `let` statements
        "let" => parse_let(rest),  

        // Handle assignments `x = 5`
        "=" => parse_assign(rest), 

        // Handle `if` conditionals
        "if" => parse_if(rest),    

        // Handle arithmetic operations
        "+" | "-" | "*" | "/" => parse_op(token, rest), 

        // Handle the `display` (print statements)
        "display" => parse_print(rest),

        // Default case for symbols or numbers
        _ => Ok((parse_atom(token), rest)),
    }
}


pub fn parse_print<'a>(tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    // The first token after `display` should be the expression to print
    let (expr, rest) = parse(tokens)?;  // Parse the expression to print
    Ok((FlareLangExp::Print(Box::new(expr)), rest))  // Create a Print expression (no "display" keyword)
}



pub fn parse_let<'a>(tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    let (var_name, rest) = tokens.split_first()
        .ok_or(FlareLangErr::Reason("expected variable name after let".to_string()))?;
    let (eq, rest) = rest.split_first()
        .ok_or(FlareLangErr::Reason("expected '=' after variable name".to_string()))?;
    if eq != "=" {
        return Err(FlareLangErr::Reason("expected '='".to_string()));
    }
    let (expr, rest) = parse(&rest)?;
    Ok((FlareLangExp::Let(var_name.clone(), Box::new(expr)), rest))
}



pub fn parse_assign<'a>(tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    let (var_name, rest) = tokens.split_first()
        .ok_or(FlareLangErr::Reason("expected variable name for assignment".to_string()))?;
    let (expr, rest) = parse(&rest)?;
    Ok((FlareLangExp::Assign(var_name.clone(), Box::new(expr)), rest))
}

pub fn parse_if<'a>(tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    let (condition, rest) = parse(&tokens)?;
    let (true_expr, rest) = parse(&rest)?;
    let (false_expr, rest) = parse(&rest)?;
    Ok((FlareLangExp::If(Box::new(condition), Box::new(true_expr), Box::new(false_expr)), rest))
}

pub fn parse_op<'a>(op: &str, tokens: &'a [String]) -> Result<(FlareLangExp, &'a [String]), FlareLangErr> {
    let (left, rest) = parse(&tokens)?;
    let (right, rest) = parse(&rest)?;
    Ok((FlareLangExp::Op(op.to_string(), Box::new(left), Box::new(right)), rest))
}

pub fn parse_atom(token: &str) -> FlareLangExp {
    if let Ok(n) = token.parse::<f64>() {
        FlareLangExp::Number(n)  // If it's a number, return it as a Number
    }
    else if let Ok(s) = token.parse:: <String> (){
        FlareLangExp::String(s)
    }
    else {
        FlareLangExp::Symbol(token.to_string())  // Otherwise, treat it as a variable or symbol
    }
}



pub fn parse_list_of_floats(args: &[FlareLangExp]) -> Result<Vec<f64>, FlareLangErr> {
    args
      .iter()
      .map(|x| parse_single_float(x))
      .collect()
  }
  
pub  fn parse_single_float(exp: &FlareLangExp) -> Result<f64, FlareLangErr> {
    match exp {
      FlareLangExp::Number(num) => Ok(*num),
      _ => Err(FlareLangErr::Reason("expected a number".to_string())),
    }
  }

