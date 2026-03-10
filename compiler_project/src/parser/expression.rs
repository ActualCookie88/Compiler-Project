use crate::token::Token;
use crate::parser::program::{ find_function, find_variable, SymbolTable};
use crate::parser::statement::create_temp;

pub struct Expression {
  pub code: String,
  pub name: String,
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
    let mut expr = parse_expression(tokens, index, table, current_func)?;

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
        name: dest
    };
    Ok(expr)
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
            
            // check if function call
            if matches!(tokens[*index], Token::LeftParen) {
                
                // SEMANTIC CHECK: "Calling a function which has not been defined"
                find_function(table, &var_name)
                    .ok_or_else(|| format!("Function '{}' is not defined", var_name))?;
                    
                *index += 1;
                
                let mut args: Vec<String> = Vec::new(); // store arguments

                // collect arguments
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
            // SEMANTIC CHECK:
            let func = find_function(table, current_func)
                .ok_or_else(|| format!("Current function '{}' not found", current_func))?;

            // SEMANTIC CHECK: "Using a variable without having declared it"
            let var_def = find_variable(func, &var_name)
                .ok_or_else(|| format!("Variable '{}' used without declaration", var_name))?;

            let is_array = var_def.is_array;

            // array indexing [
            if matches!(tokens[*index], Token::LeftBracket) {
                // SEMANTIC CHECK: "Type mismatch: using a scalar integer variable as an array of integers"
                if !is_array {
                    return Err(format!("Variable '{}' is not an array", var_name));
                }
                *index += 1;

                let index_expr = parse_expression(tokens, index, table, current_func)?;

                // ]
                match tokens[*index] {
                    Token::RightBracket => *index += 1,
                    _ => return Err(String::from("Expected ']' after array size")),
                }

                let dest = create_temp();
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

            // SEMANTIC CHECK: "Type mismatch: using an array of integers as a scalar integer"
            if var_def.is_array {
                return Err(format!("Variable '{}' is an array but used as scalar", var_name));
            }

            Ok(Expression {
                code: String::new(),
                name: var_name,
            })
        }

        // number
        Token::Num(num) => {
            *index += 1;
            Ok(Expression {
                code: String::new(),
                name: num.to_string(),
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