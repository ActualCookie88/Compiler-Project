use crate::lexer::token::Token;
use crate::parser::declaration::parse_declaration_statement;
use crate::parser::expression::{parse_expression, parse_boolean_expression};
use crate::parser::program::{find_function, find_variable, CodeGenState, LoopInfo, SymbolTable, VarKind};

use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static IF_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn create_temp() -> String {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("_temp{}", id)
}

fn create_if_label() -> (String, String, String) {
    let id = IF_COUNTER.fetch_add(1, Ordering::SeqCst);
    (
        format!("iftrue{}", id),
        format!("else{}", id),
        format!("endif{}", id),
    )
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
pub fn parse_statement(
    tokens: &[Token], 
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str ,
    state: &mut CodeGenState
) -> Result<String, String> {
    match &tokens[*index] {
        Token::Int => parse_declaration_statement(tokens, index, table, current_func),
        Token::Return => parse_return_statement(tokens, index, table, current_func),
        Token::Print => parse_print_statement(tokens, index, table, current_func),
        Token::Read => parse_read_statement(tokens, index),
        Token::If => parse_if_statement(tokens, index, table, current_func, state),
        Token::While => parse_while_statement(tokens, index, table, current_func, state),
        Token::Break => parse_break_statement(tokens, index, state),
        Token::Continue => parse_continue_statement(tokens, index, state),
        Token::Ident(_) => {
            // peek ahead: if next token is '(' this is a function call statement
            if *index + 1 < tokens.len() && matches!(tokens[*index + 1], Token::LeftParen) {
                let expr = parse_expression(tokens, index, table, current_func)?;
                match tokens[*index] {
                    Token::Semicolon => {
                        *index += 1;
                        Ok(expr.code)
                    }
                    _ => Err(String::from("Function call statement must end with ';'")),
                }
            } else {
                parse_assignment_statement(tokens, index, table, current_func)
            }
        },
        _ => Err(String::from("Invalid statement"))
    }
}

/* ////////////////////////////////////////////////////////////////////

Control Flow

//////////////////////////////////////////////////////////////////// */

fn parse_break_statement(
    tokens: &[Token],
    index: &mut usize,
    state: &mut CodeGenState
) -> Result<String, String> {
    // break
    match tokens[*index] {
        Token::Break =>  *index += 1,
        _ => return Err(String::from("Expected 'break'")),
    }

    // ;
    match tokens[*index] {
        Token::Semicolon =>  *index += 1,
        _ => return Err(String::from("Break statement must end with a semicolon")),
    }

    let loop_end = state
        .loop_stack
        .last()
        .ok_or("Used 'break' outside of a loop")?
        .end
        .clone();

    Ok(format!("%jmp {}\n", loop_end))
}

fn parse_continue_statement(
    tokens: &[Token],
    index: &mut usize,
    state: &mut CodeGenState
) -> Result<String, String> {
    // continue
    match tokens[*index] {
        Token::Continue =>  *index += 1,
        _ => return Err(String::from("Expected 'continue'")),
    }

    // ;
    match tokens[*index] {
        Token::Semicolon =>  *index += 1,
        _ => return Err(String::from("Continue statement must end with a semicolon")),
    }

    let loop_start = state
        .loop_stack
        .last()
        .ok_or("Used 'continue' outside of a loop")?
        .begin
        .clone();

    Ok(format!("%jmp {}\n", loop_start))
}

fn parse_while_statement(
    tokens: &[Token],
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str,
    state: &mut CodeGenState
) -> Result<String, String>{
    // while
    match tokens[*index] {
        Token::While => *index += 1,
        _ => return Err(String::from("Expected 'while'")),
    }

    // unique loop id for generating labels
    let loop_id = state.label_counter; 
    state.label_counter += 1;

    let loop_begin = format!(":loopbegin{}", loop_id);
    let loop_end = format!(":endloop{}", loop_id);

    // push loop info
    state.loop_stack.push(LoopInfo { begin: loop_begin.clone(), end: loop_end.clone() });

    let mut code = String::new();
    code.push_str(&format!("{}\n", loop_begin));

    let condition = parse_boolean_expression(tokens, index, table, current_func)?;
    code.push_str(&condition.code);
    code.push_str(&format!("%branch_ifn {}, {}\n", condition.name, loop_end));
    
    // {
    match tokens[*index] {
        Token::LeftCurly => *index += 1,
        _ => return Err(String::from("Expected '{' after while condition")),
    }

    // parse statements inside 'while' body
    while !matches!(tokens[*index], Token::RightCurly) {
        let before = *index;
        code.push_str(&parse_statement(tokens, index, table, current_func, state)?);

        if *index == before {
            return Err(String::from("Parser made no progress"));
        }
    }

    // }
    match tokens[*index] {
        Token::RightCurly => *index += 1,
        _ => return Err(String::from("Expected '}' to close while block")),
    }

    code.push_str(&format!("%jmp {}\n", loop_begin));
    code.push_str(&format!("{}\n", loop_end));

    // pop loop info
    state.loop_stack.pop();

    Ok(code)
}

fn parse_if_statement(
    tokens: &[Token],
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str,
    state: &mut CodeGenState
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

	// parse statements inside 'if' body
	while !matches!(tokens[*index], Token::RightCurly) {
		let before = *index;
		if_body.push_str(&parse_statement(tokens, index, table, current_func, state)?);

		if *index == before {
			return Err(String::from("Parser made no progress"));
		}
	}

	// }
	match tokens[*index] {
		Token::RightCurly =>  *index += 1,
		_ => return Err(String::from("Expected '}'")),
	}

    // (optional) else body
    let has_else = matches!(tokens[*index], Token::Else);

	if has_else {
        ir_code.push_str(&format!("%jmp :{}\n", else_label));
		*index += 1;
        
		// {
		match tokens[*index] {
			Token::LeftCurly =>  *index += 1,
			_ => return Err(String::from("Expected '{'")),
		}

		// // parse statements inside 'else' body
		while !matches!(tokens[*index], Token::RightCurly) {
			let before = *index;
			else_body.push_str(&parse_statement(tokens, index, table, current_func, state)?);
        
			if *index == before {
				return Err(String::from("Parser made no progress"));
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

/* ////////////////////////////////////////////////////////////////////

Simple Statements

//////////////////////////////////////////////////////////////////// */

fn parse_return_statement(
    tokens: &[Token],
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str
) -> Result<String, String> {
    // return
    match tokens[*index] {
        Token::Return => *index += 1,
        _ => return Err(String::from("Return statements must begin with a 'return' keyword")),
    }

    let expr = parse_expression(tokens, index, table, current_func)?;

    // ;
    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Statement must end with a semicolon")),
    }

    Ok(format!("{}%ret {}\n", expr.code, expr.name))
}

fn parse_print_statement(
    tokens: &[Token],
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str
) -> Result<String, String> {
    // print
    match tokens[*index] {
        Token::Print=> *index += 1,
        _ => return Err(String::from("Print statements must begin with 'print' keyword")),
    }

    let expr = parse_expression(tokens, index, table, current_func)?;

    // ;
    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Statements must end with a semicolon")),
    }

    Ok(format!("{}%out {}\n", expr.code, expr.name))
}

fn parse_read_statement(
    tokens: &[Token],
    index: &mut usize,
) -> Result<String, String>{
    // read
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

/* ////////////////////////////////////////////////////////////////////

Assignment

//////////////////////////////////////////////////////////////////// */

// Handles three assignment forms:
// 1. dest = src1        = %mov dest, src1
// 2. array[i] = src1    = %mov [array + i], src1
// 3. dest = array[i]    = %mov dest, [array + i]
fn parse_assignment_statement(
    tokens: &[Token],
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
    
    // Check for array indexing on lhs: array[expr]
    let lhs_index_expr = if matches!(tokens[*index], Token::LeftBracket) {
        // SEMANTIC CHECK: only arrays can be indexed
        if !matches!(lhs_var.kind, VarKind::Array(_)) {
            return Err(format!(
                "Type mismatch: scalar variable '{}' used as an array",
                ident
            ));
        }
        
        *index += 1;
        let index_expr = parse_expression(tokens, index, table, current_func)?;

        // }
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array index")),
        }
        Some(index_expr)
    } else {
        // SEMANTIC CHECK: arrays cannot be used as scalars
        if matches!(lhs_var.kind, VarKind::Array(_)) {
            return Err(format!(
                "Type mismatch: array variable '{}' used as a scalar",
                ident
            ));
        }
        None
    };

    // =
    match tokens[*index] {
        Token::Assign => *index += 1,
        _ => return Err(String::from("Missing the '=' operator")),
    }

    // right hand side of expression
    let rhs_expr = parse_expression(tokens, index, table, current_func)?;

    // Check for array indexing on rhs: base[expr]
    let rhs_index_expr = if matches!(tokens[*index], Token::LeftBracket) {
        // if "a = b[i];" rhs_expr.name should be the base variable name
        let rhs_var = find_variable(&func, &rhs_expr.name)
            .ok_or_else(|| format!("Variable '{}' used without being declared", rhs_expr.name))?;
    
         // SEMANTIC CHECK: only arrays can be indexed
        if !matches!(rhs_var.kind, VarKind::Array(_)) {
            return Err(format!(
                "Type mismatch: scalar variable '{}' used as an array",
                rhs_expr.name
            ));
        }
        
        *index += 1;
        let index_expr = parse_expression(tokens, index, table, current_func)?;

        // }
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array index")),
        }
        Some(index_expr)
    } else {
        // SEMANTIC CHECK: arrays cannot be used as scalars on the rhs either
        if let Some(rhs_var) = find_variable(&func, &rhs_expr.name) {
            if matches!(rhs_var.kind, VarKind::Array(_)) {
                return Err(format!(
                    "Type mismatch: array variable '{}' used as a scalar",
                    rhs_expr.name
                ));
            }
        }
        None
    };

    // ;
    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Missing semicolon.")),
    }

    let mut ir_code = String::new();

    // generate code 
    match (lhs_index_expr, rhs_index_expr) { 
        // array write: array[index] = rhs
        (Some(lhs_idx), None) => {
            ir_code.push_str(&lhs_idx.code);
            ir_code.push_str(&rhs_expr.code);

            let tmp = create_temp();
            ir_code.push_str(&format!("%int {}\n", tmp));
            ir_code.push_str(&format!("%mov {}, {}\n", tmp, lhs_idx.name));

            ir_code.push_str(&format!("%mov [{} + {}], {}\n", ident, tmp, rhs_expr.name));
        }
        // array read: lhs = array[index]
        (None, Some(rhs_idx)) => {
            ir_code.push_str(&rhs_expr.code);
            ir_code.push_str(&rhs_idx.code);

            let tmp = create_temp();;
            ir_code.push_str(&format!("%int {}\n", tmp));
            ir_code.push_str(&format!("%mov {}, {}\n", tmp, rhs_idx.name));

            ir_code.push_str(&format!("%mov {}, [{} + {}]\n", ident, rhs_expr.name, tmp));
        }
        // simple scalar assignment
        (None, None) => {
            ir_code.push_str(&rhs_expr.code);
            ir_code.push_str(&format!("%mov {}, {}\n", ident, rhs_expr.name));  
        }
        // edge case for Teh Tarik Language
        (Some(_), Some(_)) => {
            return Err(String::from(
                "Assignments with array indexing on both sides are not supported"
            ));
        }
    };

    Ok(ir_code)
}