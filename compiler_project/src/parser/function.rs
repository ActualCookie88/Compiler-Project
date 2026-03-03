use crate::token::Token;
use crate::parser::statement::*;
use crate::parser::program::SymbolTable;

// func main(int a, int b) {
//    # ... statements here...
// }
// a loop is done to handle statements.
pub fn parse_function(tokens: &Vec<Token>, index: &mut usize, table: &mut SymbolTable) -> Result<String, String> {
    let mut func_code = String::new();
    let mut params: Vec<String> = Vec::new();

    // func 
    match tokens[*index] {
        Token::Func => *index += 1, 
        _ => return Err(String::from("Missing the 'func' keyword.")),
    }

    // function name
    let func_name = match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;
            ident.clone()
        }
        _  => return Err(String::from("Functions must have a function identifier")),
    };

    let current_func = func_name.clone();

    crate::parser::program::add_function(table, current_func.clone())?;

    // (
    match tokens[*index] {
        Token::LeftParen => *index += 1,
        _ => return Err(String::from("Missing the left parenthesis '('")),
    }

    // parameters: int a, int b, ...
    if !matches!(tokens[*index], Token::RightParen) {
        // first param
        let param_code = parse_parameter(tokens, index)?;
        params.push(param_code);

        // more params
        while matches!(tokens[*index], Token::Comma) {
            *index += 1; 
            let param_code2 = parse_parameter(tokens, index)?;
            params.push(param_code2);
        }
    }

    // )
    match tokens[*index] {
        Token::RightParen => *index += 1, 
        _ => return Err(String::from("Missing the right parenthesis ')'")),
    }

    // generate IR
    func_code += &format!("%func {}({})\n", func_name, params.join(", "));

    // {
    match tokens[*index] {
        Token::LeftCurly => *index += 1,
        _ => return Err(String::from("Missing the left curly bracket '{'")),
    }

    // statements inside function
    while !matches!(tokens[*index], Token::RightCurly) {
        let before = *index;
        let statement_code = parse_statement(tokens, index, table, &current_func)?;
        func_code += &statement_code;

        if *index == before {
            return Err("Parser made no progress".to_string());
        }
    }

    // }
    match tokens[*index] {
        Token::RightCurly => *index += 1, 
        _ => return Err(String::from("Expected '}'")),
    }

    func_code += "%endfunc\n";

    return Ok(func_code);
}

// parameters: int a, int b, ...
pub fn parse_parameter(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    // int
    let param_type = match tokens[*index] {
        Token::Int => {
            *index += 1; 
            "int"
        }
        _ => return Err(String::from("Parameter must begin with 'int' keyword")),
    };

    // identifier
    let param_name = match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1; 
            ident.clone()
        }
        _ => return Err(String::from("Declarations must have an identifier")),
    };

    let param_code = format!("%{} {}", param_type, param_name);

    return Ok(param_code);
}