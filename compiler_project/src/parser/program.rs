use crate::token::Token;
use crate::parser::function::parse_function;

// parse programs with multiple functions
// loop over everything, outputting generated code.
pub fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));

    let mut code = String::new();
    while !at_end(tokens, *index) {
        match parse_function(tokens, index) {
            Ok(function_code) => {
                code += &function_code;
            }
            Err(e) => { return Err(e); }
        }
    }
    return Ok(code);
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
    match tokens[index] {
    Token::End => { true }
    _ => { false }
    }
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}