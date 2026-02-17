use crate::token::Token;
use crate::parser::declaration::parse_declaration_statement;
use crate::parser::expression::{parse_expression, parse_boolean_expression};
// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
pub fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {
        Token::Int => parse_declaration_statement(tokens, index),
        Token::Return => parse_return_statement(tokens, index),
        Token::Print => parse_print_statement(tokens, index),
        Token::Read => parse_read_statement(tokens, index),
        Token::If => parse_if_statement(tokens, index),
        Token::While => parse_while_statement(tokens, index),
        Token::Break => parse_break_statement(tokens, index),
        Token::Ident(_) => { // can be assignment or function call
            if *index + 1 < tokens.len() && matches!(tokens[*index + 1], Token::LeftParen) {
                parse_expression(tokens, index)?;
                match tokens[*index] {
                    Token::Semicolon => { *index += 1; Ok(()) }
                    _ => Err(String::from("Function call statement must end with ';'")),
                }
            } else {
                parse_assignment_statement(tokens, index)
            }
        },
        _ => Err(String::from("Invalid statement"))
    }
}

// break
fn parse_break_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
        Token::Break => { *index += 1; }
        _ => { return Err(String::from("Expected 'break'")); }
    }

    // ;
    match tokens[*index] {
        Token::Semicolon => { *index += 1; }
        _ => { return Err(String::from("Break statement must end with a semicolon")); }
    }

    return Ok(());
}

// while loops
fn parse_while_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {
      Token::While => *index += 1,
      _ => return Err("Expected 'while'".to_string()),
    }
    parse_boolean_expression(tokens, index)?;
    
    match tokens[*index] {
      Token::LeftCurly => *index += 1,
      _ => return Err("Expected '{' after while condition".to_string()),
    }

    while !matches!(tokens[*index], Token::RightCurly) {
        let before = *index;
        parse_statement(tokens, index)?;

        if *index == before {
            return Err("Parser made no progress".to_string());
        }
    }

    match tokens[*index] {
      Token::RightCurly => *index += 1,
       _ => return Err("Expected '}' to close while block".to_string()),
    }

    return Ok(());
}

fn parse_if_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
	// if
	match tokens[*index] {
	Token::If => {*index += 1}
	_ => {return Err(String::from("Expected 'if' keyword"));}
	}

	parse_boolean_expression(tokens, index)?;

	match tokens[*index] {
	Token::LeftCurly => { *index += 1; }
	_ => { return Err(String::from("Expected '{'"));}
	}

	// parse body until }
	while !matches!(tokens[*index], Token::RightCurly) {
		let before = *index;
		parse_statement(tokens, index)?;

		if *index == before {
			return Err("Parser made no progress".to_string());
		}
	}

	// }
	match tokens[*index] {
		Token::RightCurly => { *index += 1; }
		_ => { return Err(String::from("Expected '}'"));}
	}

	if matches!(tokens[*index], Token::Else) {
		*index += 1;
		// {
		match tokens[*index] {
			Token::LeftCurly => { *index += 1; }
			_ => { return Err(String::from("Expected '{'"));}
		}

		// parse body until }
		while !matches!(tokens[*index], Token::RightCurly) {
			let before = *index;
			parse_statement(tokens, index)?;

			if *index == before {
				return Err("Parser made no progress".to_string());
			}
		}

		// }
		match tokens[*index] {
			Token::RightCurly => { *index += 1; }
			_ => { return Err(String::from("Expected '}'"));}
		}
	}

	return Ok(())
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {
      Token::Return => {*index += 1;}
      _ => {return Err(String::from("Return statements must begin with a 'return' keyword"));}
    }

    parse_expression(tokens, index)?;

    match tokens[*index] {
      Token::Semicolon => {*index += 1;}
      _ => {return Err(String::from("Statement must end with a semicolon"));}
    }

    return Ok(());
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {
    Token::Print=> {*index += 1;}
    _ => {return Err(String::from("Print statements must begin with 'print' keyword"));}
    }

    parse_expression(tokens, index)?;

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Statements must end with a semicolon"));}
    }

    return Ok(());
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {
    Token::Read => {*index += 1;}
    _ => {return Err(String::from("Read statements must begin with a 'read' keyword"));}
    }

    parse_expression(tokens, index)?;

    match tokens[*index] {
      Token::Semicolon => {*index += 1;}
      _ => {return Err(String::from("Statement is missing the ';' semicolon"));}
    }

    return Ok(());
}

fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
    match tokens[*index] {
    Token::Ident(_) => {*index += 1;}
    _ => {return Err(String::from("Assignment statements must begin with an identifier"));}
    }

    // Check for array indexing: identifier[expression]
    if matches!(tokens[*index], Token::LeftBracket) {
        *index += 1;
        parse_expression(tokens, index)?;
        match tokens[*index] {
        Token::RightBracket => {*index += 1;}
        _ => {return Err(String::from("Expected ']' after array index"));}
        }
    }

    match tokens[*index] {
    Token::Assign => {*index += 1;}
    _ => {return Err(String::from("Missing the '=' operator"));}
    }

    parse_expression(tokens, index)?;

    match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Missing semicolon."));}
    }

    return Ok(());
}