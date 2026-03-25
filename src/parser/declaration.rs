use crate::lexer::token::Token;
use crate::parser::program::{SymbolTable, Variable, add_local};
use crate::parser::expression::{parse_expression, Expression};

// int a;   int [8] a;
pub fn parse_declaration_statement(
    tokens: &Vec<Token>,
    index: &mut usize,
    table: &mut SymbolTable,
    current_func: &str
    ) -> Result<String, String> {
    // int
    match tokens[*index] {
        Token::Int => {*index += 1;}
        _ => {return Err(String::from("Declaration statements must begin with 'int' keyword"));}
    }

    let mut array_size: Option<i32> = None;
    
    // [number]
    if matches!(tokens[*index], Token::LeftBracket) {
        *index += 1;

        // number / array size
        match tokens[*index] {
            Token::Num(num) => {
                *index += 1;
                array_size = Some(num);
            }
            _ => return Err(String::from("Expected number within '[]")),
        }
        
        // ]
        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Expected ']' after array size")),
        }
    }

    // identifier
    let var_name = match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;
            ident.clone()
        }
        _ => return Err(String::from("Declarations must have an identifier")),
    };

    // check if declaration is array or scalar, and add to symbol table accordingly
    let mut ir_code = String::new();
    match array_size {
        Some(size) => {
            if size <= 0 { // array size semantic check
                return Err(String::from("Array size must be greater than 0"));
            }

            add_local(table, current_func, Var {
                name: var_name.clone(),
                is_array: true,
                size,
            })?;

            match tokens[*index] {
                Token::Semicolon => *index += 1,
                _ => return Err("Declaration must end with ';'".to_string()),
            }

            Ok(format!("%int[] {}, {}\n", var_name, size))
        }

        //scalar
        None => {
            add_local(table, current_func, Variable {
                name: var_name.clone(),
                is_array: false,
                size: 0,
            })?;

            ir_code.push_str(&format!("%int {}\n", var_name));
            
            if matches!(tokens[*index], Token::Assign) {
                *index += 1;
                let rhs_expr: Expression = parse_expression(tokens, index, table, current_func)?;
                
                // append all IR code for the expression
                ir_code.push_str(&rhs_expr.code);
                
                // move the result into the variable
                ir_code.push_str(&format!("%mov {}, {}\n", var_name, rhs_expr.name));
            }

            match tokens[*index] {
                Token::Semicolon => *index += 1,
                _ => return Err("Declaration must end with ';'".to_string()),
            }

            Ok(ir_code)
        }
    }
}