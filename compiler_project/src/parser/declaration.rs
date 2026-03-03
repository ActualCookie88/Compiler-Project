use crate::token::Token;

// int a;   int [8] a;
pub fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
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

    // ;
    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Expected ';' at end of declaration")),
    }

    // generate code
    let ir_code = match array_size {
        Some(size) => format!("%int[] {}, {}\n", var_name, size),
        None => format!("%int {}\n", var_name),
    };

    Ok(ir_code)
}