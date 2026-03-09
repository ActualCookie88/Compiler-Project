use crate::token::Token;
use crate::parser::declaration::parse_declaration_statement;
use crate::parser::expression::{parse_expression, parse_boolean_expression};
use crate::parser::program::{SymbolTable, find_function, find_variable};

static mut TEMP_COUNTER: i64 = 0;
static mut IF_COUNTER: i64 = 0;

fn create_temp() -> String {
    unsafe {
        TEMP_COUNTER += 1;
        format!("_temp{}", TEMP_COUNTER)
    }
}

fn create_if_label() -> (String, String, String) {
    unsafe {
        IF_COUNTER += 1;
        let if_true = format!("iftrue{}", IF_COUNTER);
        let else_label = format!("else{}", IF_COUNTER);
        let end_if = format!("endif{}", IF_COUNTER);
        (if_true, else_label, end_if)
    }
}
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
    let mut ir_code = String::new();
    let mut if_body = String::new();
    let mut else_body = String::new();

    let condition = parse_boolean_expression(tokens, index, table, current_func)?;

    ir_code.push_str(&condition.code);

    // labels
    let (if_label, else_label, end_if_label) = create_if_label();
    
    // branch to if
    ir_code.push_str(&format!("%branch_if {}, :{}\n", condition.name, if_label));

    // {
	match tokens[*index] {
        Token::LeftCurly =>  *index += 1,
        _ => return Err(String::from("Expected '{'")),
	}

	// parse body until }
	while !matches!(tokens[*index], Token::RightCurly) {
		let before = *index;
		if_body.push_str(&parse_statement(tokens, index, table, current_func)?);

		if *index == before {
			return Err("Parser made no progress".to_string());
		}
	}

	// }
	match tokens[*index] {
		Token::RightCurly =>  *index += 1,
		_ => return Err(String::from("Expected '}'")),
	}

    // else statement
    let mut has_else = false;
	if matches!(tokens[*index], Token::Else) {
        has_else = true;
        ir_code.push_str(&format!("%jmp :{}\n", else_label));
		*index += 1;
		// {
		match tokens[*index] {
			Token::LeftCurly =>  *index += 1,
			_ => return Err(String::from("Expected '{'")),
		}

		// parse body until }
		while !matches!(tokens[*index], Token::RightCurly) {
			let before = *index;
			else_body.push_str(&parse_statement(tokens, index, table, current_func)?);
        
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

    // Generate IR
    if has_else {
        ir_code.push_str(&format!(":{}\n", if_label));
        ir_code.push_str(&if_body);
        ir_code.push_str(&format!("%jmp :{}\n", end_if_label));

        ir_code.push_str(&format!(":{}\n", else_label));
        ir_code.push_str(&else_body);
    }
    else {
        ir_code.push_str(&format!("%jmp :{}\n", end_if_label));
        ir_code.push_str(&format!(":{}\n", if_label));
        ir_code.push_str(&if_body);
    }

    ir_code.push_str(&format!(":{}\n", end_if_label));

    Ok(ir_code)
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

    // func must be declared
    let func = find_function(table, current_func)
        .ok_or_else(|| format!("Function '{}' not found", current_func))?
        .clone();

    // lhs must be declared
    let lhs_var = find_variable(&func, &ident)
        .ok_or_else(|| format!("Variable '{}' not declared in function '{}'", ident, current_func))?;
    
    // Check for array indexing on lhs: [expression]
    let lhs_index_expr = if matches!(tokens[*index], Token::LeftBracket) {
        // lhs must be an array
        if !lhs_var.is_array {
            return Err(format!(
                "Type mismatch: scalar integer variable '{}' used as an array",
                ident
            ));
        }
        
        *index += 1;
        let index_expr = parse_expression(tokens, index, table, current_func)?;
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array index")),
        }
        Some(index_expr)
    } else {
        // semantic check: if no indexing on lhs, lhs must be scalar
        if lhs_var.is_array {
            return Err(format!(
                "Type mismatch: array integer variable '{}' used as a scalar",
                ident
            ));
        }        
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
        // if "a = b[i];" rhs_expr.name should be the base variable name
        let rhs_var = find_variable(&func, &rhs_expr.name)
            .ok_or_else(|| format!("Variable '{}' used without being declared", rhs_expr.name))?;
    
        // rhs base must be array
        if !rhs_var.is_array {
            return Err(format!(
                "Type mismatch: scalar integer variable '{}' used as an array",
                rhs_expr.name
            ));
        }
        
        *index += 1;
        let index_expr = parse_expression(tokens, index, table, current_func)?;
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array index")),
        }
        Some(index_expr)
    } else {
        // if rhs expression is just a variable name and that variable is an array,
        // then array is being used like a scalar
        if let Some(rhs_var) = find_variable(&func, &rhs_expr.name) {
            if rhs_var.is_array {
                return Err(format!(
                    "Type mismatch: array integer variable '{}' used as a scalar",
                    rhs_expr.name
                ));
            }
        }
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
            ir_code.push_str(&format!("%mov {}, {}\n", ident, rhs_expr.name));  // simple assignment
        }
        (Some(_), Some(_)) => {
            return Err(String::from(
                "Assignments with array indexing on both sides are not supported"
            ));
        }
    };

    Ok(ir_code)
}