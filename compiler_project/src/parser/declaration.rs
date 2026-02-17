use crate::token::Token;
use crate::parser::expression::parse_expression;
// int a;   int a = 0;   int a = b;
pub fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    // int
    match tokens[*index] {
      Token::Int => {*index += 1;}
      _ => {return Err(String::from("Declaration statements must begin with 'int' keyword"));}
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
      Token::Ident(_) => {*index += 1;}
      _ => {return Err(String::from("Declarations must have an identifier"));}
    }

    // [expression]
    if matches!(tokens[*index], Token::LeftBracket) {
      *index += 1;

      parse_expression(tokens, index)?;

      match tokens[*index] {
          Token::RightBracket => { *index += 1; }
          _ => { return Err(String::from("Expected ']' after array size")); }
      }
  }

    // ; or =
    match tokens[*index] {
      Token::Semicolon => {
        *index += 1; 
        return Ok(());
      }
  
      Token::Assign => {*index += 1;}
      _ => {return Err(String::from("Expected ';' or '=' after identifier"));}
    }

    // number or identifier
    match tokens[*index] {
      Token::Num(_) | Token::Ident(_) => {*index += 1;}  
      _ => {return Err(String::from("Expected number or identifier after '='"));}
    }

    // ;
    match tokens[*index] {
      Token::Semicolon => {
        *index += 1;
        Ok(())
      }
      _ => {return Err(String::from("Statement must end with a semicolon"));}
    }
}