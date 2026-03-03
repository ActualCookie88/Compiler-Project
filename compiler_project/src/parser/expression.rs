use crate::token::Token;

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
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expr = parse_multiply_expression(tokens, index)?;
    loop {
        let operation = match tokens[*index] {
            Token::Plus => "add",
            Token::Subtract => "sub",
            _ => break,
       };
       *index += 1;

        let expr2 = parse_multiply_expression(tokens, index)?;
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

pub fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let expr = parse_expression(tokens, index)?;

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

    let expr2 = parse_expression(tokens, index)?;
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

pub fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut expr = parse_term(tokens, index)?;

    loop {
       let operation = match tokens[*index] {
            Token::Multiply => "mult",
            Token::Divide => "div",
            Token::Modulus => "mod",
            _ => break,
        };
        *index += 1;

        let expr2 = parse_term(tokens, index)?;
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
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;

            let mut code = String::new();
            let var_name = ident.clone();

            // array indexing
            if matches!(tokens[*index], Token::LeftBracket) {
                *index += 1;

                let index_expr = parse_expression(tokens, index)?;
                code = format!("{}{}", code, index_expr.code);

                match tokens[*index] {
                    Token::RightBracket => *index += 1,
                    _ => return Err(String::from("Expected ']' after array size")),
                }
            }
            
            // function call
            if matches!(tokens[*index], Token::LeftParen) {
                *index += 1;

                while !matches!(tokens[*index], Token::RightParen) {
                    let arg_expr = parse_expression(tokens, index)?;
                    code = format!("{}{}", code, arg_expr.code);

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
            }

            Ok(Expression { 
                code, 
                name: var_name 
            })
        }

        Token::Num(num) => {
            *index += 1;
            Ok(Expression {
                code: String::new(),
                name: num.to_string() 
            })
        }

        Token::LeftParen => {
            *index += 1;
            let expr = parse_expression(tokens, index)?;

            match tokens[*index] {
                Token::RightParen => *index += 1,
                _ => return Err(String::from("Missing the right parenthesis ')'")),
            }
            return Ok(expr);
        }
        
        _ => return Err(String::from("Invalid Expression.")),

    }
}