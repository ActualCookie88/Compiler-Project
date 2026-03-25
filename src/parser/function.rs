use crate::lexer::token::Token;
use crate::parser::statement::*;
use crate::parser::program::{
    add_function, add_param, CodeGenState, SymbolTable, Variable, VarKind,
};

// func main(int a, int b) {
//    # ... statements here...
// }
// a loop is done to handle statements.
pub fn parse_function(
    tokens: &[Token],
    index: &mut usize,
    table: &mut SymbolTable,
    state: &mut CodeGenState
) -> Result<String, String> {
    let mut func_code = String::new();
    let mut params: Vec<String> = Vec::new();

    // func 
    match tokens[*index] {
        Token::Func => *index += 1, 
        _ => return Err(String::from("Missing the 'func' keyword.")),
    }

    // function name (identifier)
    let func_name = match &tokens[*index] {
        Token::Ident(ident) => {
            *index += 1;
            ident.clone()
        }
        _  => return Err(String::from("Functions must have a function identifier")),
    };

    add_function(table, func_name.clone())?;

    // (
    match tokens[*index] {
        Token::LeftParen => *index += 1,
        _ => return Err(String::from("Missing the left parenthesis '('")),
    }

    // parameters: int a, int b, ...
    if !matches!(tokens[*index], Token::RightParen) {
        // first param
        let (param_name, param_code) = parse_parameter(tokens, index)?;
        params.push(param_code);

        add_param(table, &func_name, Variable { name: param_name, kind: VarKind::Scalar })?;

        // remaining params
        while matches!(tokens[*index], Token::Comma) {
            *index += 1; 
            let (param_name, param_code) = parse_parameter(tokens, index)?;
            params.push(param_code);
            add_param(table, &func_name, Variable { name: param_name, kind: VarKind::Scalar })?;
        }
    }

    // )
    match tokens[*index] {
        Token::RightParen => *index += 1, 
        _ => return Err(String::from("Missing the right parenthesis ')'")),
    }

    // generate IR header
    func_code.push_str(&format!("%func {}({})\n", func_name, params.join(", ")));

    // {
    match tokens[*index] {
        Token::LeftCurly => *index += 1,
        _ => return Err(String::from("Missing the left curly bracket '{'")),
    }

    // statements inside function body
    while !matches!(tokens[*index], Token::RightCurly) {
        let before = *index;
        let statement_code = parse_statement(tokens, index, table, &func_name, state)?;
        func_code.push_str(&statement_code);

        if *index == before {
            return Err(String::from("Parser made no progress"));
        }
    }

    // }
    match tokens[*index] {
        Token::RightCurly => *index += 1, 
        _ => return Err(String::from("Expected '}'")),
    }

    func_code.push_str("%endfunc\n");
 
    Ok(func_code)
}

// Parse a single parameter of the form 'int <name>'.
// Returns '(param_name, ir_code)'.
pub fn parse_parameter(
    tokens: &[Token],
    index: &mut usize
) -> Result<(String, String), String> {
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
        _ => return Err(String::from("Parameter must have an identifier")),
    };

    let param_code = format!("%{} {}", param_type, param_name);

    return Ok((param_name, param_code));
}