use crate::token::Token;
use crate::parser::statement::*;
// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.
pub fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    
    match tokens[*index] {
    Token::Func => { *index += 1; }
    _ => { return Err(String::from("Missing the 'func' keyword.")); }
    }

    match tokens[*index] {
    Token::Ident(_) => { *index += 1; }
    _  => { return Err(String::from("Functions must have a function identifier"));}
    }

    match tokens[*index] {
    Token::LeftParen => { *index += 1; }
    _ => { return Err(String::from("Missing the left parenthesis '('"));}
    }

    if !matches!(tokens[*index], Token::RightParen) {
        // first param
        parse_declaration_statement_for_function(tokens, index)?;

        // more params
        while matches!(tokens[*index], Token::Comma) {
            *index += 1; 
            parse_declaration_statement_for_function(tokens, index)?;
        }
    }

    match tokens[*index] {
    Token::RightParen => { *index += 1; }
    _ => { return Err(String::from("Missing the right parenthesis ')'"));}
    }

    match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("Missing the left curly bracket '{'"));}
    }

    while !matches!(tokens[*index], Token::RightCurly) {
        let before = *index;
        parse_statement(tokens, index)?;

        if *index == before {
            return Err("Parser made no progress".to_string());
        }
    }

    match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("Expected '}'"));}
    }

    return Ok(());
}

pub fn parse_declaration_statement_for_function(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    // int
    match tokens[*index] {
        Token::Int => { *index += 1; }
        _ => { return Err(String::from("Declaration statements must begin with 'int' keyword")); }
    }

    // [expression]
    if matches!(tokens[*index], Token::LeftBracket) {
      *index += 1;

      match tokens[*index] {
          Token::Num(_) => { *index += 1; }
          _ => { return Err(String::from("Expected number within")); }
      }

      match tokens[*index] {
          Token::RightBracket => { *index += 1; }
          _ => { return Err(String::from("Expected ']' after array size")); }
      }

    }

    // identifier
    match tokens[*index] {
        Token::Ident(_) => { *index += 1; }
        _ => { return Err(String::from("Declarations must have an identifier")); }
    }

    return Ok(());
}