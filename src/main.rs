// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.

/* HOW TO RUN EXAMPLES
   From the project root:
   cargo run -- <examples_name>/<filename.tt>
   e.g. cargo run -- examples_ir2/break.tt
*/
use std::env;
use std::fs;

use compiler_project::lexer::lex;
use compiler_project::parser::program::parse_program;

mod interpreter;

fn main() {
    // get commandline arguments
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => { println!("Please provide an input file."); return; }
        2 => {}
        _ => { println!("Too many commandline arguments."); return; }
    }

    // read the entire file.
    let filename = &args[1];

    let code = match fs::read_to_string(filename) {
        Ok(code) => code,
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }
    };

    // lexer
    let tokens = match lex(&code) {
        Ok(t) => t,
        Err(e) => {
            println!("Lexer Error: {}", e);
            return;
        }
    };
    // Uncomment to print out tokens
    // for t in &tokens {
    //     println!("{:?}", t);
    // }

    // parser
    let mut index = 0;
    match parse_program(&tokens, &mut index) {
        Ok(generated_code) => {
            println!("Successfully Parsed The Code.");
            println!("{}", generated_code);
            interpreter::execute_ir(&generated_code);
        }
        Err(e) => {
            println!("Parser Error: {}", e);
        }
    }
}

/* ////////////////////////////////////////////////////////////////////

Tests

//////////////////////////////////////////////////////////////////// */

// Writing tests shows robustness and helps catch regressions early.
// Run all tests with: 'cargo test'
#[cfg(test)]
mod tests {
    use compiler_project::lexer::lex;
    use compiler_project::lexer::token::Token;
    use compiler_project::parser::program::parse_program;

    // Lexer
 
    #[test]
    fn lexer_numbers_and_operators() {
        let toks = lex("1 + 2 + 3").unwrap();
        assert_eq!(toks.len(), 6);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));
        assert!(matches!(toks[5], Token::End));
 
        // trailing operator is fine, lexer should still tokenize
        let toks = lex("3 + 215 +").unwrap();
        assert_eq!(toks.len(), 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::End));
    }
 
    #[test]
    fn lexer_keywords() {
        let toks = lex("func return int print read while if else break continue").unwrap();
        assert!(matches!(toks[0],  Token::Func));
        assert!(matches!(toks[1],  Token::Return));
        assert!(matches!(toks[2],  Token::Int));
        assert!(matches!(toks[3],  Token::Print));
        assert!(matches!(toks[4],  Token::Read));
        assert!(matches!(toks[5],  Token::While));
        assert!(matches!(toks[6],  Token::If));
        assert!(matches!(toks[7],  Token::Else));
        assert!(matches!(toks[8],  Token::Break));
        assert!(matches!(toks[9],  Token::Continue));
        assert!(matches!(toks[10], Token::End));
    }
 
    #[test]
    fn lexer_delimiters() {
        let toks = lex("( ) { } [ ] , ;").unwrap();
        assert!(matches!(toks[0], Token::LeftParen));
        assert!(matches!(toks[1], Token::RightParen));
        assert!(matches!(toks[2], Token::LeftCurly));
        assert!(matches!(toks[3], Token::RightCurly));
        assert!(matches!(toks[4], Token::LeftBracket));
        assert!(matches!(toks[5], Token::RightBracket));
        assert!(matches!(toks[6], Token::Comma));
        assert!(matches!(toks[7], Token::Semicolon));
        assert!(matches!(toks[8], Token::End));
    }
 
    #[test]
    fn lexer_arithmetic_operators() {
        let toks = lex("+ - * / %").unwrap();
        assert!(matches!(toks[0], Token::Plus));
        assert!(matches!(toks[1], Token::Subtract));
        assert!(matches!(toks[2], Token::Multiply));
        assert!(matches!(toks[3], Token::Divide));
        assert!(matches!(toks[4], Token::Modulus));
        assert!(matches!(toks[5], Token::End));
    }
 
    #[test]
    fn lexer_comparison_operators() {
        let toks = lex("= < <= > >= == !=").unwrap();
        assert!(matches!(toks[0], Token::Assign));
        assert!(matches!(toks[1], Token::Less));
        assert!(matches!(toks[2], Token::LessEqual));
        assert!(matches!(toks[3], Token::Greater));
        assert!(matches!(toks[4], Token::GreaterEqual));
        assert!(matches!(toks[5], Token::Equality));
        assert!(matches!(toks[6], Token::NotEqual));
        assert!(matches!(toks[7], Token::End));
    }
 
    #[test]
    fn lexer_identifiers() {
        let toks = lex("var_1 = 32;").unwrap();
        assert!(matches!(&toks[0], Token::Ident(s) if s == "var_1"));
        assert!(matches!(toks[1], Token::Assign));
        assert!(matches!(toks[2], Token::Num(32)));
        assert!(matches!(toks[3], Token::Semicolon));
        assert!(matches!(toks[4], Token::End));
    }
 
    #[test]
    fn lexer_rejects_invalid_tokens() {
        assert!(lex("^^^").is_err());
    }
 
    // Parser
 
    #[test]
    fn parse_simple_program() {
        let code = "
            func main() {
                int a;
                int b;
                a = 1 + 2;
                b = a * 3;
            }
        ";
        let tokens = lex(code).unwrap();
        assert!(parse_program(&tokens, &mut 0).is_ok());
    }
 
    #[test]
    fn parse_if_and_while() {
        let code = "
            func main() {
                int a;
                a = 5;
                if a > 0 {
                    a = a - 1;
                }
            }
        ";
        let tokens = lex(code).unwrap();
        assert!(parse_program(&tokens, &mut 0).is_ok());
    }
 
    #[test]
    fn parse_rejects_missing_semicolon() {
        let code = "
            func main() {
                int a;
                a = 5
            }
        ";
        let tokens = lex(code).unwrap();
        assert!(parse_program(&tokens, &mut 0).is_err());
    }
 
    #[test]
    fn parse_rejects_missing_main() {
        let code = "
            func foo() {
                int a;
            }
        ";
        let tokens = lex(code).unwrap();
        assert!(parse_program(&tokens, &mut 0).is_err());
    }

    #[test]
    fn parse_rejects_undeclared_variable() {
        let code = "
            func main() {
                x = 5;
            }
        ";
        let tokens = lex(code).unwrap();
        let result = parse_program(&tokens, &mut 0);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(
            msg.contains("not declared") || msg.contains("used without declaration"),
            "unexpected error: {}", msg
        );
    }

    #[test]
    fn parse_rejects_break_outside_loop() {
        let code = "
            func main() {
                int a;
                a = 1;
                break;
            }
        ";
        let tokens = lex(code).unwrap();
        let result = parse_program(&tokens, &mut 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("outside of a loop"));
    }

    #[test]
    fn parse_rejects_continue_outside_loop() {
        let code = "
            func main() {
                int a;
                a = 1;
                continue;
            }
        ";
        let tokens = lex(code).unwrap();
        let result = parse_program(&tokens, &mut 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("outside of a loop"));
    }
}

