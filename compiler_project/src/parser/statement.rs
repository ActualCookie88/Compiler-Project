use crate::token::Token;
use crate::parser::declaration::parse_declaration_statement;
use crate::parser::expression::{parse_expression, parse_boolean_expression, create_temp};
use crate::parser::program::{SymbolTable};
// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
pub fn parse_statement(
        tokens: &Vec<Token>, 
        index: &mut usize,
        table: &mut SymbolTable,
        current_func: &str ) -> Result<String, String> {
    match &tokens[*index] {
        Token::Int => parse_declaration_statement(tokens, index, table, current_func),
        Token::Return => parse_return_statement(tokens, index, table, current_func),
        Token::Print => parse_print_statement(tokens, index, table, current_func),
        Token::Read => parse_read_statement(tokens, index),
        Token::If => parse_if_statement(tokens, index, table, current_func),
        Token::While => parse_while_statement(tokens, index, table, current_func),
        Token::Break => parse_break_statement(tokens, index),
        Token::Ident(_) => {
            if *index + 1 < tokens.len() && matches!(tokens[*index + 1], Token::LeftParen) { // function call
                let expr = parse_expression(tokens, index, table, current_func)?;
                match tokens[*index] {
                    Token::Semicolon => {
                        *index += 1;
                        Ok(expr.code)
                    }
                    _ => Err(String::from("Function call statement must end with ';'")),
                }
            } else { // assignment
                parse_assignment_statement(tokens, index, table, current_func)
            }
        },
        _ => Err(String::from("Invalid statement"))
    }
}

// break
fn parse_break_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    match tokens[*index] {
        Token::Break =>  *index += 1,
        _ => return Err(String::from("Expected 'break'")),
    }

    // ;
    match tokens[*index] {
        Token::Semicolon =>  *index += 1,
        _ => return Err(String::from("Break statement must end with a semicolon")),
    }

    return Ok(String::new())
}

// while loops
fn parse_while_statement(
        tokens: &Vec<Token>,
        index: &mut usize,
        table: &mut SymbolTable,
        current_func: &str
    ) -> Result<String, String>{
    match tokens[*index] {
        Token::While => *index += 1,
        _ => return Err("Expected 'while'".to_string()),
    }
    parse_boolean_expression(tokens, index, table, current_func)?;
    
    match tokens[*index] {
        Token::LeftCurly => *index += 1,
        _ => return Err("Expected '{' after while condition".to_string()),
    }

    while !matches!(tokens[*index], Token::RightCurly) {
        let before = *index;
        parse_statement(tokens, index, table, current_func)?;

        if *index == before {
            return Err("Parser made no progress".to_string());
        }
    }

    match tokens[*index] {
        Token::RightCurly => *index += 1,
        _ => return Err("Expected '}' to close while block".to_string()),
    }

    return Ok(String::new())
}

fn parse_if_statement(
        tokens: &Vec<Token>,
        index: &mut usize,
        table: &mut SymbolTable,
        current_func: &str
    ) -> Result<String, String>{
	// if
	match tokens[*index] {
	    Token::If => *index += 1,
	    _ => return Err(String::from("Expected 'if' keyword")),
	}

    // (
    match tokens[*index] {
        Token::LeftParen => *index += 1,
        _ => return Err(String::from("Expected '(' after if")),
    }

	parse_boolean_expression(tokens, index, table, current_func)?;

    // )
    match tokens[*index] {
        Token::RightParen => *index += 1,
        _ => return Err(String::from("Missing the right parenthesis ')'")),
    }

    // {
	match tokens[*index] {
        Token::LeftCurly =>  *index += 1,
        _ => return Err(String::from("Expected '{'")),
	}

	// parse body until }
	while !matches!(tokens[*index], Token::RightCurly) {
		let before = *index;
		parse_statement(tokens, index, table, current_func)?;

		if *index == before {
			return Err("Parser made no progress".to_string());
		}
	}

	// }
	match tokens[*index] {
		Token::RightCurly =>  *index += 1,
		_ => return Err(String::from("Expected '}'")),
	}

	if matches!(tokens[*index], Token::Else) {
		*index += 1;
		// {
		match tokens[*index] {
			Token::LeftCurly =>  *index += 1,
			_ => return Err(String::from("Expected '{'")),
		}

		// parse body until }
		while !matches!(tokens[*index], Token::RightCurly) {
			let before = *index;
			parse_statement(tokens, index, table, current_func)?;

			if *index == before {
				return Err("Parser made no progress".to_string());
			}
		}

		// }
		match tokens[*index] {
			Token::RightCurly =>  *index += 1,
			_ => return Err(String::from("Expected '}'")),
		}
	}

	return Ok(String::new())
}

fn parse_return_statement(
        tokens: &Vec<Token>,
        index: &mut usize,
        table: &mut SymbolTable,
        current_func: &str
    ) -> Result<String, String> {
    match tokens[*index] {
        Token::Return => *index += 1,
        _ => return Err(String::from("Return statements must begin with a 'return' keyword")),
    }

    let expr = parse_expression(tokens, index, table, current_func)?;

    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Statement must end with a semicolon")),
    }

    return Ok(format!("{}%ret {}\n", expr.code, expr.name))
}

fn parse_print_statement(
        tokens: &Vec<Token>,
        index: &mut usize,
        table: &mut SymbolTable,
        current_func: &str
    ) -> Result<String, String> {
    match tokens[*index] {
        Token::Print=> *index += 1,
        _ => return Err(String::from("Print statements must begin with 'print' keyword")),
    }

    let expr = parse_expression(tokens, index, table, current_func)?;

    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Statements must end with a semicolon")),
    }

    let ir_code = format!("{}%out {}\n", expr.code, expr.name);
    return Ok(ir_code)
}

fn parse_read_statement(
        tokens: &Vec<Token>,
        index: &mut usize,
    ) -> Result<String, String>{
    match tokens[*index] {
        Token::Read => *index += 1,
        _ => return Err(String::from("Read statements must begin with a 'read' keyword")),
    }

    let name = match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;
            ident.clone()
        }
        _ => return Err(String::from("Read expects an identifier")),
    };

    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Statement is missing the ';' semicolon")),
    }

    Ok(format!("%input {}\n", name))
}

/// 1. dest = src1        = %mov dest, src1
/// 2. array[i] = src1    = %mov [array + i], src1
/// 3. dest = array[i]    = %mov dest, [array + i]
fn parse_assignment_statement(
    tokens: &Vec<Token>,
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str
) -> Result<String, String> {
    // identifier
    let ident = match &tokens[*index] {
        Token::Ident(identifier) => {
            *index += 1;
            identifier.clone()
        }
        _ => return Err(String::from("Assignment statements must begin with an identifier")),
    };

    // Check for array indexing on lhs: [expression]
    let lhs_index_expr = if matches!(tokens[*index], Token::LeftBracket) {
        *index += 1;
        let index_expr = parse_expression(tokens, index, table, current_func)?;
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array index")),
        }
        Some(index_expr)
    } else {
        None
    };

    // = operator
    match tokens[*index] {
        Token::Assign => *index += 1,
        _ => return Err(String::from("Missing the '=' operator")),
    }

    // right hand side of expression
    let rhs_expr = parse_expression(tokens, index, table, current_func)?;

    // Check for array indexing on rhs: [expression]
    let rhs_index_expr = if matches!(tokens[*index], Token::LeftBracket) {
        *index += 1;
        let index_expr = parse_expression(tokens, index, table, current_func)?;
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array index")),
        }
        Some(index_expr)
    } else {
        None
    };

    // semicolon ;
    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Missing semicolon.")),
    }

    let mut ir_code = String::new();

    // generate code 
    match (lhs_index_expr, rhs_index_expr) { 
        // array write: array[index] = rhs
        (Some(lhs_expr), None) => {
            ir_code.push_str(&lhs_expr.code);
            ir_code.push_str(&rhs_expr.code);

            let lhs_temp = create_temp();
            ir_code.push_str(&format!("%int {}\n", lhs_temp));
            ir_code.push_str(&format!("%mov {}, {}\n", lhs_temp, lhs_expr.name));

            ir_code.push_str(&format!("%mov [{} + {}], {}\n", ident, lhs_temp, rhs_expr.name));
        }
        // array read: lhs = rhs[index]
        (None, Some(rhs_index)) => {
            ir_code.push_str(&rhs_expr.code);
            ir_code.push_str(&rhs_index.code);

            let rhs_temp = create_temp();
            ir_code.push_str(&format!("%int {}\n", rhs_temp));
            ir_code.push_str(&format!("%mov {}, {}\n", rhs_temp, rhs_index.name));

            ir_code.push_str(&format!("%mov {}, [{} + {}]\n", ident, rhs_expr.name, rhs_temp));
        }
        (None, None) => {
            ir_code.push_str(&rhs_expr.code);
            ir_code.push_str(&format!("%mov {}, {}\n", ident, rhs_expr.name));                      // simple assignment
        }
        (Some(_), Some(_)) => return Err(String::from("Assignments with array indexing on both sides are not supported")),
    };

    Ok(ir_code)
}