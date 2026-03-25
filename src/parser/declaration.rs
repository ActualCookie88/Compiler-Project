use crate::lexer::token::Token;
use crate::parser::program::{add_local, SymbolTable, Variable, VarKind};
use crate::parser::expression::{parse_expression, Expression};

// int a;   int [8] a;
pub fn parse_declaration_statement(
    tokens: &[Token],
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str
) -> Result<String, String> {
    // int
    match tokens[*index] {
        Token::Int => {*index += 1;}
        _ => {return Err(String::from("Declaration statements must begin with 'int' keyword"));}
    }

    // optional [size] for array declarations
    let array_size: Option<usize> = if matches!(tokens[*index], Token::LeftBracket) {
        *index += 1;

        // number / array size
        let size = match tokens[*index] {
            Token::Num(num) => {
                *index += 1;
                num as usize
            }
            _ => return Err(String::from("Expected number within '[]")),
        };
        
        // }
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array size")),
        }
        Some(size)
    } else {
        None
    };

    // identifier
    let var_name = match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;
            ident.clone()
        }
        _ => return Err(String::from("Declarations must have an identifier")),
    };

    // add to symbol table and generate IR
    let mut ir_code = String::new();

    match array_size {
        // array declaration: int [n] a;
        Some(size) => {
            add_local(table, current_func, Variable {
                name: var_name.clone(),
                kind: VarKind::Array(size),
            })?;
 
            match tokens[*index] {
                Token::Semicolon => *index += 1,
                _ => return Err(String::from("Declaration must end with ';'")),
            }
 
            ir_code.push_str(&format!("%int[] {}, {}\n", var_name, size));
        }

        // scalar declaration: int a;  or  int a = expr;
        None => {
            add_local(table, current_func, Variable {
                name: var_name.clone(),
                kind: VarKind::Scalar,
            })?;
 
            ir_code.push_str(&format!("%int {}\n", var_name));
 
            // optional initializer
            if matches!(tokens[*index], Token::Assign) {
                *index += 1;
                let rhs: Expression = parse_expression(tokens, index, table, current_func)?;
                ir_code.push_str(&rhs.code);
                ir_code.push_str(&format!("%mov {}, {}\n", var_name, rhs.name));
            }
            
            // ;
            match tokens[*index] {
                Token::Semicolon => *index += 1,
                _ => return Err(String::from("Declaration must end with ';'")),
            }
        }
    }
    Ok(ir_code)
}