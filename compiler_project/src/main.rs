// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

fn main() {
    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
        code
    } 

    };

    let tokens = match lex(&code) {
    Err(error_message) => {
        println!("**Error**");
        println!("----------------------");
        println!("{}", error_message);
        println!("----------------------");
        return;
    }

    Ok(data) => data,
    
    };


    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("File Contents:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }

}

// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, Clone)]
enum Token {
    Func,
    Return,
    Int,
    Print,
    Read, 
    While,
    If,
    Else,
    Break,
    Continue,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Plus,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Assign,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    NotEqual,
    Num(i32),
    Ident(String),
    End,
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


// This is a lexer that parses numbers and math operations
fn lex(code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;

    match c {
    // whitespace 
    ' ' | '\n' => {
      i += 1;
    }
    // comments
    '#' => {
      i += 1;
      while i < bytes.len() && bytes[i] as char != '\n' {
          i += 1;
      }
    }

    '(' => {
      tokens.push(Token::LeftParen);
      i += 1;
    }

    ')' => {
      tokens.push(Token::RightParen);
      i += 1;
    }

    '{' => {
      tokens.push(Token::LeftCurly);
      i += 1;
    }

    '}' => {
      tokens.push(Token::RightCurly);
      i += 1;
    }

    '[' => {
      tokens.push(Token::LeftBracket);
      i += 1;
    }

    ']' => {
      tokens.push(Token::RightBracket);
      i += 1;
    }

    ',' => {
      tokens.push(Token::Comma);
      i += 1;
    }

    ';' => {
      tokens.push(Token::Semicolon);
      i += 1;
    }

    '+' => {
      tokens.push(Token::Plus);
      i += 1;
    }

    '-' => {
      tokens.push(Token::Subtract);
      i += 1;
    }

    '*' => {
      tokens.push(Token::Multiply);
      i += 1;
    }

    '/' => {
      tokens.push(Token::Divide);
      i += 1;
    }

    '%' => {
      tokens.push(Token::Modulus);
      i += 1;
    }

    // = and ==
    '=' => {
      i += 1;
      if i < bytes.len() && bytes[i] as char == '=' {
        tokens.push(Token::Equality);
        i += 1;
      } else {
        tokens.push(Token::Assign);
      }
    }

    // != or error if just !
    '!' => {
      i += 1;
      if i < bytes.len() && bytes[i] as char == '=' {
          tokens.push(Token::NotEqual);
          i += 1;
      } else {
          return Err("Unrecognized symbol '!'".to_string());
      }
    }

    // < and <=
    '<' => {
      i += 1;
      if i < bytes.len() && bytes[i] as char == '=' {
        tokens.push(Token::LessEqual);
        i += 1;
      } else {
        tokens.push(Token::Less);
      }
    }

    // > and >=
    '>' => {
      i += 1;
      if i < bytes.len() && bytes[i] as char == '=' {
        tokens.push(Token::GreaterEqual);
        i += 1;
      } else {
        tokens.push(Token::Greater);
      }
    }

    // identifiers
    'a'..='z' | 'A'..='Z' | '_' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let letter = bytes[i] as char; 
        if letter.is_alphabetic() || letter == '_' || letter.is_numeric() {
          i += 1;
        } else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let token = create_identifier(string_token);
      tokens.push(token)
    }

    // numbers
    '0'..='9' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit >= '0' && digit <= '9' {
          i += 1;
        } else if digit.is_alphabetic() {
          return Err(format!("Invalid identifier: {}", &code[start..i]));
        } else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let number_value = string_token.parse::<i32>().unwrap();
      let token = Token::Num(number_value);
      tokens.push(token);
    }

    _ => {
      return Err(format!("Unrecognized symbol '{}'", c));
    }

    }
  }

  tokens.push(Token::End);
  return Ok(tokens);
}

fn create_identifier(code: &str) -> Token {
  match code {
    "func" => Token::Func,
    "return" => Token::Return,
    "int" => Token::Int,
    "print" => Token::Print,
    "read" => Token::Read,
    "while" => Token::While,
    "if" => Token::If,
    "else" => Token::Else,
    "break" => Token::Break,
    "continue" => Token::Continue,
    _ => Token::Ident(String::from(code)),
  }
}

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

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
}