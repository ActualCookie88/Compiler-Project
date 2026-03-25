# Phase 2: Parser (Syntax Analysis)

## Overview
In this phase, we implement a **parser** for the Teh Tarik programming language.

A parser takes the sequence of tokens created by the lexer and creates a structural representation of the program while checking for correct syntax.

If the lexer identifies the “words” and “punctuation” of the language, the parser identifies the “sentences” and overall structure (functions, loops, conditionals, etc.).

## Objectives
- Validate program syntax
- Identify program structures (functions, statements, expressions)
- Detect and report syntax errors
- Prepare for code generation (Phase 3)

## Parser Grammer

A parser follows a **context-free grammar (CFG)**. You can find the proper grammar of the Teh Tarik Programming Lanuage [here](https://cs.ucr.edu/~dtan004/CS152_Parsing.pdf)

## What is a Parser?

A parser determines what a sequence of tokens represents. For example:
```bash
identifier = number;
```
This sequence would be recognized as an **assignment statement**.

The parser ensures that:
- Tokens appear in the correct order
- Required tokens exist (e.g., semicolons, parentheses)
- Structures are valid

## Building a Parser

We will be building a simple top down recursive descent parser without backtracking. Let's start with a simple declaration statement `int a;`. We can parse a simple declaration statement with the following pseudocode:

```
parse_declaration_statement(tokens: Array<Token>) -> Result(value,Error) {
    t := next_token(tokens)?
    if t != INT KEYWORD,
        return Err("expected integer keyword")

    t := next_token(tokens)?
    if t != IDENTIFIER,
        return Err("expected identifier")
    
    t := next_token(tokens)?
    if t != SEMICOLON,
        return Err("expected semicolon ';'")

    return Success
}
```

A simple declaration statement is a sequence of an `integer keyword`, followed by an `identifier`, followed by a `semicolon`. This simple pseudo code checks that the `tokens` has the specificed sequence.

## Branching Parser Behavior

Programming Language Grammars can have branching behavior that allows for expressive power. For example, there are multiple possibilities `int a;` , `int a = 0;`, or `int a = b;`. Here is some pseudocode to parse that grammar:

```
parse_declaration_statement(tokens: Array<Token>) -> Result(value,Error) {
    t := next_token(tokens)?
    if t != INT KEYWORD,
        return Err("expected integer keyword")

    t := next_token(tokens)?
    if t != IDENTIFIER,
        return Err("expected identifier")
    
    t := next_token(tokens)?
    if t == SEMICOLON,
        return Success

    if t == EQUAL {
       t := next_token(tokens)?
       if t == NUMBER
            t := next_token(tokens)?
            if t == SEMICOLON,
                return Success
            else
                return Err("expected semicolon ';'")
            
       if t == IDENTIFIER
            t := next_token(tokens)?
            if t == SEMICOLON,
                return Success
            else
                return Err("expected semicolon ';'")

       return Err("expected number or identifier")
    }

    return Err("expected semicolon ';' or '=' assignment operator")
}
```

**In Phase 2, when you call a function when doing top-down recursive descent parsing, make sure to propagate the error back up the calling stack so that the error can be caught correctly.**

## matches!() statement

[matches macro](https://doc.rust-lang.org/std/macro.matches.html)

This macro returns `true` when the two parameters are equivalent, and returns `false` when the two parameters
are not equivalent.

```
let token = Token::Func;
if matches!(token, Token::Func) {
    println!("True");
} else {
    println!("False");
}
```

## Lifetimes (Optional. Not needed for this assignment)

[Lifetimes Documentation](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)

## ? Operator

Documentation: [? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used)

The `?` operator is an error propagation operation. If result of the operation causes an error, the execution of the program
stops and the error value is returned. In order for `?` operator to function correctly, the error value type **must** match the 
the function it is returning from. If the result is not an error, the `?` operator unwraps the `Result` or `Option` automatically
for you. This technique can simplify the error handling code in Rust.

**In Phase 2, when you call a function, make sure to propagate the error back up the calling stack so that the error can be caught correctly.**

For example, if you have two functions `parse_function` and `parse_statement`, you can simplify the match statements using the `?` operator in the following way:
```
fn parse_function(tokens: &[Token], index: &mut usize) -> Result<(), String> {

    // 1.) this is the conventional match statement
    match parse_statement(tokens, index) {
        Ok(()) => {}
        Err(e) => {return Err(e);}
    }

    // 2.) this is another shorthand equivalent way to write 1.) 
    parse_statement(tokens, index)?;
}

fn parse_statement(tokens: &[Token], index: &mut usize) -> Result<(), String> {
    todo!()
}
```
This only works when the Result Error return values match.

## Building a Top Down Parser

Start by creating a function called `parse_program`. It will take in a list of tokens and index marking where the parser is.
It will return a return a `Result`, where `Result` can either be `Err` or it will be fine.
```
// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &[Token], index: &mut usize) -> Result<(), String> {
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
    while !at_end(tokens, *index) {
      match parse_function(tokens, index) {
      Ok(()) => {}
      Err(e) => { return Err(e); }
      }
    }
    return Ok(());
}
```

A program consists of multiple functions, and we loop over the tokens, parsing out the functions.

We then create another function called `parse_function` that will parse the functions.

Assuming that the function grammar is as follows:
```
func main() {
    // insert statements here...
}
```
We can write `parse_function` like this:

```
fn parse_function(tokens: &[Token], index: &mut usize) -> Result<(), String> {
    
    match tokens[*index] {
    Token::Func => { *index += 1; }
    _ => { return Err(String::from("functions must begin with func")); }
    }

    match tokens[*index] {
    Token::Ident(_) => { *index += 1; }
    _  => { return Err(String::from("functions must have a function identifier"));}
    }


    match tokens[*index] {
    Token::LeftParen => { *index += 1; }
    _ => { return Err(String::from("expected '('"));}
    }

    match tokens[*index] {
    Token::RightParen => { *index += 1; }
    _ => { return Err(String::from("expected ')'"));}
    }

    match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("expected '{'"));}
    }

    while !matches!(tokens[*index], Token::RightCurly) {

        match parse_statement(tokens, index) {
        Ok(()) => {}
        Err(e) => {return Err(e);}
        }
    }

    match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("expected '}'"));}
    }

    return Ok(());
}
```

Writing `parse_statement` follows a similar pattern to `parse_function` and `parse_program`. You can
modify the `parse_expression` example to make it into a statement.

## Requirements

The parser must:
- Accept valid grammar
- Reject invalid grammar
- Detect missing tokens
- Ensure balanced structures

## Test Cases

- add.tt
- array.tt
- break.tt
- function.tt
- if.tt
- loop.tt
- math.tt
- nested_loop.tt
