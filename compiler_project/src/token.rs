// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, Clone)]
pub enum Token {
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