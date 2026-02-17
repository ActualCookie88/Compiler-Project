use crate::token::Token;
use crate::parser::function::parse_function;
// parse programs with multiple functions
// loop over everything, outputting generated code.
pub fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
    while !at_end(tokens, *index) {
      parse_function(tokens, index)?;
    }
    return Ok(());
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
  match tokens[index] {
  Token::End => { true }
  _ => { false }
  }
}