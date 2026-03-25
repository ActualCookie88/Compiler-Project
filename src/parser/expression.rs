use crate::lexer::token::Token;
use crate::parser::program::{find_function, find_variable, SymbolTable, VarKind};
use crate::parser::statement::create_temp;

pub struct Expression {
  pub code: String,
  pub name: String,
}

// Combines two already-evaluated expressions with a binary operator into a new temp.
fn combine(left: Expression, right: Expression, op: &str) -> Expression {
    let dest = create_temp();
    Expression {
        code: format!(
            "{}{}%int {}\n%{} {}, {}, {}\n",
            left.code, right.code, dest, op, dest, left.name, right.name
        ),
        name: dest,
    }
}

/* ////////////////////////////////////////////////////////////////////

Expression Parsers

//////////////////////////////////////////////////////////////////// */

// Parses additive expressions: 'term ((+ | -) term)*'
// e.g. a + b - (c * d) / (f + g - 8);
pub fn parse_expression(
    tokens: &[Token], 
    index: &mut usize,
    table: &mut SymbolTable, 
    current_func: &str
) -> Result<Expression, String> {
    let mut expr = parse_multiply_expression(tokens, index, table, current_func)?;

    loop {
        let op = match tokens[*index] {
            Token::Plus     => "add",
            Token::Subtract => "sub",
            _ => break,
        };
        *index += 1;
 
        let right = parse_multiply_expression(tokens, index, table, current_func)?;
        expr = combine(expr, right, op);
    }
 
    Ok(expr)
}

// Parse a boolean (comparison) expression: 'expr <comparison_operator> expr'
pub fn parse_boolean_expression(
    tokens: &[Token], 
    index: &mut usize, 
    table: &mut SymbolTable, 
    current_func: &str
) -> Result<Expression, String> {
    let left= parse_expression(tokens, index, table, current_func)?;

    let op = match tokens[*index] {
        Token::Less         => "lt",
        Token::LessEqual    => "le",
        Token::Greater      => "gt",
        Token::GreaterEqual => "ge",
        Token::Equality     => "eq",
        Token::NotEqual     => "ne",
        _ => return Err(String::from("Expected a comparison operator")),
    };
    *index += 1;
 
    let right = parse_expression(tokens, index, table, current_func)?;
    Ok(combine(left, right, op))
}

// Parse a multiplicative expression: `term ((* | / | %) term)*`
pub fn parse_multiply_expression(
    tokens: &[Token], 
    index: &mut usize, 
    table: &mut SymbolTable, 
    current_func: &str
) -> Result<Expression, String> {
    let mut expr = parse_term(tokens, index, table, current_func)?;

    loop {
        let op = match tokens[*index] {
            Token::Multiply => "mult",
            Token::Divide   => "div",
            Token::Modulus  => "mod",
            _ => break,
        };
        *index += 1;
 
        let right = parse_term(tokens, index, table, current_func)?;
        expr = combine(expr, right, op);
    }
 
    Ok(expr)
}

/* ////////////////////////////////////////////////////////////////////

Term Parser

//////////////////////////////////////////////////////////////////// */

// A term is a function call, a variable reference, an array index, a number literal, or a parenthesised expression.
fn parse_term(
    tokens: &[Token], 
    index: &mut usize, 
    table: &mut SymbolTable, 
    current_func: &str
) -> Result<Expression, String> {
    match &tokens[*index] {
        // variable / identifier
        Token::Ident(ident) => {
            *index += 1;
            let var_name = ident.clone();
            
            // function call: name(...) ──────────────────────────────────────────────────────
            if matches!(tokens[*index], Token::LeftParen) {
                
                // SEMANTIC CHECK: calling an undefined function
                find_function(table, &var_name)
                    .ok_or_else(|| format!("Function '{}' is not defined", var_name))?;
                    
                *index += 1;
                
                let mut arg_code = String::new();
                let mut args: Vec<String> = Vec::new();

                // collect arguments
                while !matches!(tokens[*index], Token::RightParen) {
                    let arg = parse_expression(tokens, index, table, current_func)?;
                    arg_code.push_str(&arg.code);
                    args.push(arg.name);

                    // consume ,
                    if matches!(tokens[*index], Token::Comma) {
                        *index += 1;
                    } else {
                        break;
                    }
                }

                // )
                match tokens[*index] {
                    Token::RightParen => *index += 1,
                    _ => { return Err(String::from("Expected ')' after function call")); }
                }

                let dest = create_temp();
                return Ok(Expression {
                    code: format!(
                        "{}%int {}\n%call {}, {}({})\n",
                        arg_code, dest, dest, var_name, args.join(",")
                    ),
                    name: dest,
                });
            }

            // variable reference ────────────────────────────────────────────────────────────
            // SEMANTIC CHECK: using a variable without declaring it
            let func = find_function(table, current_func)
                .ok_or_else(|| format!("Current function '{}' not found", current_func))?;

            let var_def = find_variable(func, &var_name)
                .ok_or_else(|| format!("Variable '{}' used without declaration", var_name))?;

            let is_array = matches!(var_def.kind, VarKind::Array(_));

            // array indexing: name[expr] ────────────────────────────────────────────────────
            if matches!(tokens[*index], Token::LeftBracket) {
                // SEMANTIC CHECK: scalar used as array
                if !is_array {
                    return Err(format!("Variable '{}' is not an array", var_name));
                }
                *index += 1;

                let index_expr = parse_expression(tokens, index, table, current_func)?;

                // ]
                match tokens[*index] {
                    Token::RightBracket => *index += 1,
                    _ => return Err(String::from("Expected ']' after array size")),
                }

                let dest = create_temp();
                return Ok(Expression {
                    code: format!(
                        "{}%int {}\n%mov {}, [{} + {}]\n",
                        index_expr.code, dest, dest, var_name, index_expr.name
                    ),
                    name: dest,
                });
            }

            // SEMANTIC CHECK: array used as scalar
            if is_array {
                return Err(format!("Variable '{}' is an array but used as a scalar", var_name));
            }

            Ok(Expression { code: String::new(), name: var_name })
        }

        // number literal
        Token::Num(num) => {
            *index += 1;
            Ok(Expression { code: String::new(), name: num.to_string() })
        }

        // parenthesised expression: (expr)
        Token::LeftParen => {
            *index += 1;
            let expr = parse_expression(tokens, index, table, current_func)?;

            match tokens[*index] {
                Token::RightParen => *index += 1,
                _ => return Err(String::from("Missing the right parenthesis ')'")),
            }
            Ok(expr)
        }
        
        _ => Err(String::from("Invalid Expression.")),

    }
}