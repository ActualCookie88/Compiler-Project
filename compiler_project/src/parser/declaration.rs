use crate::token::Token;
use crate::parser::program::{SymbolTable, Var, add_function, add_param, add_local};

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

            Ok(format!("%int[] {}, {}\n", var_name, size))
        }

        //scalar
        None => {
            add_local(table, current_func, Var {
                name: var_name.clone(),
                is_array: false,
                size: 0,
            })?;

            Ok(format!("%int {}\n", var_name))
        }
    }
}