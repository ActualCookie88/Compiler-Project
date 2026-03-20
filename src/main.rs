// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.

/* HOW TO RUN EXAMPLES
1. ./cs152.sh
2. cd compiler_project/src/
4. cargo run -- examples/(.tt filename)

*/
use std::env; // used to get the commandline arguments from the commandline.
use std::fs; // used to interact with the file system

use compiler_project::lexer::lex;
use compiler_project::parser::program::parse_program;

mod interpreter;

fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let code = match result {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => {
            code
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

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use compiler_project::lexer::token::Token;
    use compiler_project::lexer::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 6);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));
        assert!(matches!(toks[5], Token::End));

        let toks = lex("3 + 215 +").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::End));
        // keywords
        let toks = lex("func return int print read while if else break continue").unwrap();
        assert!(matches!(toks[0], Token::Func));
        assert!(matches!(toks[1], Token::Return));
        assert!(matches!(toks[2], Token::Int));
        assert!(matches!(toks[3], Token::Print));
        assert!(matches!(toks[4], Token::Read));
        assert!(matches!(toks[5], Token::While));
        assert!(matches!(toks[6], Token::If));
        assert!(matches!(toks[7], Token::Else));
        assert!(matches!(toks[8], Token::Break));
        assert!(matches!(toks[9], Token::Continue));
        assert!(matches!(toks[10], Token::End));
        // ( ) { } [ ] , ;
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
        // operators
        let toks = lex("+ - * / %").unwrap();
        assert!(matches!(toks[0], Token::Plus));
        assert!(matches!(toks[1], Token::Subtract));
        assert!(matches!(toks[2], Token::Multiply));
        assert!(matches!(toks[3], Token::Divide));
        assert!(matches!(toks[4], Token::Modulus));
        assert!(matches!(toks[5], Token::End));
        // comparison 
        let toks = lex("= < <= > >= == !=").unwrap();
        assert!(matches!(toks[0], Token::Assign));
        assert!(matches!(toks[1], Token::Less));
        assert!(matches!(toks[2], Token::LessEqual));
        assert!(matches!(toks[3], Token::Greater));
        assert!(matches!(toks[4], Token::GreaterEqual));
        assert!(matches!(toks[5], Token::Equality));
        assert!(matches!(toks[6], Token::NotEqual));
        assert!(matches!(toks[7], Token::End));

        let toks = lex("var_1 = 32;").unwrap();
        assert!(matches!(toks[0], Token::Ident(ref s) if s == "var_1"));
        assert!(matches!(toks[1], Token::Assign));
        assert!(matches!(toks[2], Token::Num(32)));
        assert!(matches!(toks[3], Token::Semicolon));
        assert!(matches!(toks[4], Token::End));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));

    }

    #[test]
    fn parse_test() {
        use compiler_project::parser::program::parse_program;
        use compiler_project::lexer::lex;

        // Valid program
        let code = "
            func main() {
                int a;
                int b;
                a = 1 + 2;
                b = a * 3;
            }
        ";

        let tokens = lex(code).unwrap();
        let mut index = 0;

        let result = parse_program(&tokens, &mut index);
        assert!(result.is_ok());

        // Program with if/while
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
        let mut index = 0;

        assert!(parse_program(&tokens, &mut index).is_ok());

        // Missing semicolon, should fail
        let code = "
            func main() {
                int a;
                a = 5
            }
        ";

        let tokens = lex(code).unwrap();
        let mut index = 0;

        assert!(parse_program(&tokens, &mut index).is_err());

        // No main function, should fail (semantic check)
        let code = "
            func foo() {
                int a;
            }
        ";

        let tokens = lex(code).unwrap();
        let mut index = 0;

        assert!(parse_program(&tokens, &mut index).is_err());

    }
}