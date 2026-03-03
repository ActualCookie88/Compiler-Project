use crate::token::Token;

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
};

struct Expression {
  code: String,
  name: String,
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    parse_multiply_expression(tokens, index)?;
    loop {
       match tokens[*index] {

       Token::Plus => {
           *index += 1;
           parse_multiply_expression(tokens, index)?;
       }

       Token::Subtract => {
           *index += 1;
           parse_multiply_expression(tokens, index)?;
       }

       _ => { 
           break;
       }

       };
    }

    return Ok(());
}

pub fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    parse_expression(tokens, index)?;

    match tokens[*index] {
        Token::Less
        | Token::LessEqual
        | Token::Greater
        | Token::GreaterEqual
        | Token::Equality
        | Token::NotEqual => {
            *index += 1;
            // match parse_multiply_expression(tokens, index) {

            
        }
        _ => {return Err(String::from("Expected a comparison operator"));}
    }
    parse_expression(tokens, index)?;

    return Ok(());
    }

pub fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    parse_term(tokens, index)?;

    loop {
       match tokens[*index] {
        Token::Multiply | Token::Divide | Token::Modulus=> {
          *index += 1;
          parse_term(tokens, index)?;
       }

       _ => {
           break;
       }

       };

    }

    return Ok(());
}

// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {

    Token::Ident(_) => {
        *index += 1;

        // array indexing
        if matches!(tokens[*index], Token::LeftBracket) {
          *index += 1;

          parse_expression(tokens, index)?;

          match tokens[*index] {
              Token::RightBracket => { *index += 1; }
              _ => { return Err(String::from("Expected ']' after array size")); }
          }
        }
        
        if matches!(tokens[*index], Token::LeftParen) {
          *index += 1;

          while !matches!(tokens[*index], Token::RightParen) {
            parse_expression(tokens, index)?;
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

        return Ok(());
    }

    Token::Num(_) => {
        *index += 1;
        return Ok(());
    }

    Token::LeftParen => {
        *index += 1;
        parse_expression(tokens, index)?;

        match tokens[*index] {
        Token::RightParen => {*index += 1;}
        _ => { return Err(String::from("Missing the right parenthesis ')'")); }
        }
        return Ok(());
    }
    
    _ => {
        return Err(String::from("Invalid Expression."));
    }

    }
}