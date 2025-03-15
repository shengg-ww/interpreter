
#[derive(Clone)]

pub enum FlareLangErr {
    Reason(String),
  }
  
 
#[derive(Clone)]
 
pub enum FlareLangExp {
  Symbol(String),
  Number(f64),
  String(String),
  List(Vec<FlareLangExp>),
  Print(Box<FlareLangExp>),
  Func(fn(&[FlareLangExp]) -> Result<FlareLangExp, FlareLangErr>), 
  Let(String, Box<FlareLangExp>),
  Assign(String, Box<FlareLangExp>),
  If(Box<FlareLangExp>, Box<FlareLangExp>, Box<FlareLangExp>),  // For conditional expressions
  Op(String, Box<FlareLangExp>, Box<FlareLangExp>),  // For arithmetic operations (e.g., +, -, *)
}


pub fn tokenize(expr: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut temp_token = String::new();
    let mut in_string = false;

    for c in expr.chars() {
        match c {
            '"' => {
                if in_string {
                    // End of the string token
                    tokens.push(temp_token.clone());
                    temp_token.clear();
                }
                in_string = !in_string; // Toggle string state
            }
            ' ' => {
                if !in_string {
                    if !temp_token.is_empty() {
                        tokens.push(temp_token.clone()); // Push the token and reset
                        temp_token.clear();
                    }
                } else {
                    temp_token.push(c); // Add space within a string (to preserve it)
                }
            }
            _ => {
                temp_token.push(c); // Add character to the current token
            }
        }
    }

    // If there is any remaining token at the end of the loop
    if !temp_token.is_empty() {
        tokens.push(temp_token);
    }

    tokens
}


