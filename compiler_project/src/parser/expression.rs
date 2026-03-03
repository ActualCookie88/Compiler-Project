use crate::token::Token;
use crate::parser::program::{ find_function, find_variable, SymbolTable};

pub struct Expression {
  pub code: String,
  pub name: String,
}

static mut VAR_NUM: i64 = 0;

pub fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize, table: &mut SymbolTable, current_func: &str) -> Result<Expression, String> {
    let mut expr = parse_multiply_expression(tokens, index, table, current_func)?;
    loop {
        let operation = match tokens[*index] {
            Token::Plus => "add",
            Token::Subtract => "sub",
            _ => break,
       };
       *index += 1;

        let expr2 = parse_multiply_expression(tokens, index, table, current_func)?;
        let dest = create_temp();

        // Combine IR
        expr = Expression {
            code: format!("{}{}%int {}\n%{} {}, {}, {}\n",
                expr.code,     // previous code
                expr2.code,    // next operand code
                dest,          // new temp
                operation,     // "add" or "sub"
                dest,          // destination temp
                expr.name,     // left operand
                expr2.name     // right operand
            ),
            name: dest,
        };
    }

    Ok(expr)
}

pub fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize, table: &mut SymbolTable, current_func: &str) -> Result<Expression, String> {
    let expr = parse_expression(tokens, index, table, current_func)?;

    let operation = match tokens[*index] {
        Token::Less => "lt",
        Token::LessEqual => "le",
        Token::Greater => "gt",
        Token::GreaterEqual => "ge",
        Token::Equality => "eq",
        Token::NotEqual => "ne",
        _ => return Err(String::from("Expected a comparison operator")),
    };
    *index += 1;

    let expr2 = parse_expression(tokens, index, table, current_func)?;
    let dest = create_temp();

    // Combine IR
    let code = format!("{}{}%int {}\n%{} {}, {}, {}\n",
        expr.code,
        expr2.code,
        dest,
        operation,
        dest,
        expr.name,
        expr2.name
    );

    Ok(Expression {
        code, 
        name: dest
    })
}

pub fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize, table: &mut SymbolTable, current_func: &str) -> Result<Expression, String> {
    let mut expr = parse_term(tokens, index, table, current_func)?;

    loop {
       let operation = match tokens[*index] {
            Token::Multiply => "mult",
            Token::Divide => "div",
            Token::Modulus => "mod",
            _ => break,
        };
        *index += 1;

        let expr2 = parse_term(tokens, index, table, current_func)?;
        let dest = create_temp();

        // Combine IR
        expr = Expression {
            code: format!("{}{}%int {}\n%{} {}, {}, {}\n",
                expr.code,
                expr2.code,
                dest,
                operation,
                dest,
                expr.name,
                expr2.name
            ),
            name: dest,
        };
    }

    Ok(expr)
}

// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize, table: &mut SymbolTable, current_func: &str) -> Result<Expression, String> {
    match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;

            let mut code = String::new();
            let var_name = ident.clone();
            
            // check if function
            if let Some(func) = find_function(table, &var_name) {
                if !matches!(tokens[*index], Token::LeftParen) {
                    return Err(format!("Function '{}' must be called with parentheses", var_name));
                }
                *index += 1;
                let mut args: Vec<String> = Vec::new();

                while !matches!(tokens[*index], Token::RightParen) {
                    let arg_expr = parse_expression(tokens, index, table, current_func)?;
                    code = format!("{}{}", code, arg_expr.code);
                    args.push(arg_expr.name);

                    if matches!(tokens[*index], Token::Comma) {
                        *index += 1;
                    } else {
                        break;
                    }
                }

                match tokens[*index] {
                    Token::RightParen => { *index += 1; }
                    _ => { return Err(String::from("Expected ')' after function call")); }
                }

                let dest = create_temp();
                
                return Ok(Expression {
                    code: format!(
                        "{}%int {}\n%call {}, {}({})\n",
                        code,
                        dest,
                        dest,
                        var_name,
                        args.join(",")
                    ),
                    name: dest,
                });
            }

            // Otherwise is variable
            let mut is_array = false;
            for func in &table.functions {
                if let Some(var) = find_variable(func, &var_name) {
                    is_array = var.is_array;
                    break;
                }
            }

            // array indexing
            if matches!(tokens[*index], Token::LeftBracket) {
                if !is_array {
                    return Err(format!("Variable '{}' is not an array", var_name));
                }
                *index += 1;

                let index_expr = parse_expression(tokens, index, table, current_func)?;
                let dest = create_temp();

                match tokens[*index] {
                    Token::RightBracket => *index += 1,
                    _ => return Err(String::from("Expected ']' after array size")),
                }
                return Ok(Expression {
                    code: format!(
                        "{}%int {}\n%mov {}, [{} + {}]\n",
                        index_expr.code,
                        dest,
                        dest,
                        var_name,
                        index_expr.name
                    ),
                    name: dest,
                });
            }

            Ok(Expression {
                code: String::new(),
                name: var_name,
            })
        }

        // number
        Token::Num(num) => {
            *index += 1;
            let dest = create_temp();

            Ok(Expression {
                code: format!(
                    "%int {}\n%mov {}, {}\n",
                    dest, dest, num
                ),
                name: dest,
            })
        }

        // (expression)
        Token::LeftParen => {
            *index += 1;
            let expr = parse_expression(tokens, index, table, current_func)?;

            match tokens[*index] {
                Token::RightParen => *index += 1,
                _ => return Err(String::from("Missing the right parenthesis ')'")),
            }
            Ok(expr)
        }
        
        _ => Err(String::from("Invalid Expression.")),

    }
}