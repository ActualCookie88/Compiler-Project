use crate::token::Token; // import token enum

// This is a lexer that parses numbers and math operations
pub fn lex(code: &str) -> Result<Vec<Token>, String> {
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
    'a'..='z' | 'A'..='Z' => {
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
          return Err(format!("Invalid identifier: {}", &code[start..=i]));
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