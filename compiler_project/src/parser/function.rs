use crate::token::Token;
use crate::parser::statement::*;
use crate::parser::program::{SymbolTable, Var, add_function, add_param};

// func main(int a, int b) {
//    # ... statements here...
// }
// a loop is done to handle statements.
pub fn parse_function(
        tokens: &Vec<Token>, 
        index: &mut usize,
        table: &mut SymbolTable) -> Result<String, String> {
    
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

    add_function(table, func_name.clone())?; // so if we func add(int a) we know if it eists
    // (
    match tokens[*index] {
        Token::LeftParen => *index += 1,
        _ => return Err(String::from("Missing the left parenthesis '('")),
    }

    // parameters: int a, int b, ...
    if !matches!(tokens[*index], Token::RightParen) {
        // first param
        let param_code = parse_parameter(tokens, index, table, &func_name)?;
        params.push(param_code);

        // more params
        while matches!(tokens[*index], Token::Comma) {
            *index += 1; 
            let param_code2 = parse_parameter(tokens, index, table, &func_name)?;
            params.push(param_code2);
        }
    }

    // )
    match tokens[*index] {
        Token::RightParen => *index += 1, 
        _ => return Err(String::from("Missing the right parenthesis ')'")),
    }

    if func_name == "main" && !params.is_empty() {
        return Err(String::from("main function cannot have parameters"));
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
        let statement_code = parse_statement(tokens, index, table, &func_name)?;
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
pub fn parse_parameter(
        tokens: &Vec<Token>, 
        index: &mut usize,
        table: &mut SymbolTable,
        func_name: &str
        ) -> Result<String, String> {
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

    let var = Var {
        name: param_name.clone(),
        is_array: false,
        size: 0,
    };
    add_param(table, func_name, var)?;

    return Ok(format!("%int {}", param_name));
}