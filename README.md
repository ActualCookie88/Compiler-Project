# Teh Tarik Compiler

A handwritten compiler for the **Teh Tarik** programming language, implemented in Rust. Built as part of CS152: Compiler Design at the University of California, Riverside. The compiler translates Teh Tarik source files (`.tt`) into a custom intermediate representation (IR), which is then executed by an interpreter.

> The language is named after Teh Tarik, the national drink of Malaysia.

---

## Features

- **Handwritten lexer** — single-pass tokenizer with precise error messages for invalid tokens (e.g. malformed identifiers like `2a`, unrecognized symbols)
- **Recursive descent parser** — top-down, single-pass parser with no backtracking; validates program structure and propagates descriptive syntax errors
- **One-pass IR code generation** — directly emits IR during parsing with no intermediate AST; handles arithmetic, function calls, control flow, and arrays
- **Operator precedence** — expression parsing is stratified across `parse_expression` → `parse_multiply_expression` → `parse_term`, correctly handling precedence without a grammar table
- **Symbol table with semantic checks** — tracks declared variables and functions per scope; catches use-before-declaration, scalar/array type mismatches, calls to undefined functions, and `break`/`continue` outside of loops at compile time
- **Control flow** — generates labeled IR for `while` loops, `if`/`else` branches, `break`, `continue`, and arbitrarily nested combinations
- **Arrays** — supports fixed-size integer arrays with index expressions in both lvalue and rvalue positions
- **Function calls** — supports multi-function programs with typed parameters and return values
- **Test suite** — regression tests covering the lexer and parser, runnable with `cargo test`

---

## Language Overview

Teh Tarik is a simple, statically typed imperative language. Below is a summary of supported features.

| Feature               | Syntax                  |
|-----------------------|-------------------------|
| Variable declaration  | `int x;`                |
| Array declaration     | `int [8] arr;`          |
| Assignment            | `x = expr;`             |
| Arithmetic            | `+ - * / %`             |
| Comparison            | `< <= > >= == !=`       |
| Print                 | `print(x);`             |
| Read                  | `read(x);`              |
| While loop            | `while cond { ... }`    |
| If / else             | `if cond { } else { }`  |
| Functions             | `func name(int a) { }`  |
| Return                | `return expr;`          |
| Break / Continue      | `break;` / `continue;`  |
| Comments              | `# single line`         |

Identifiers must begin with a letter (A–Z or a–z) and may contain letters, digits, and underscores.

---

## Project Structure

```
compiler-project/
├── src/
│   ├── lexer/
│   │   ├── lexer.rs        # Tokenizer
│   │   ├── token.rs        # Token enum
│   │   └── mod.rs
│   ├── parser/
│   │   ├── program.rs      # Entry point, symbol table, parse_program
│   │   ├── function.rs     # Function parsing and parameter handling
│   │   ├── statement.rs    # Statement dispatch and control flow codegen
│   │   ├── declaration.rs  # Variable and array declaration parsing
│   │   ├── expression.rs   # Expression parsing with precedence
│   │   └── mod.rs
│   ├── interpreter/
│   │   └── interpreter.rs  # IR interpreter (provided by course)
│   ├── lib.rs
│   └── main.rs
├── examples_lexer/         # Lexer test programs (.tt)
├── examples_parser/        # Parser test programs (.tt)
├── examples_ir1/           # Phase 3 codegen test programs (.tt)
├── examples_ir2/           # Phase 4 codegen test programs (.tt)
├── Cargo.lock
└── Cargo.toml
```

---

## How to Build and Run

### Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started) (install via `rustup`)

### Build

```bash
cargo build
```

### Run

```bash
cd src
cargo run -- <path/to/file.tt>
```

**Examples:**

```bash
cargo run -- examples_ir2/loop.tt
cargo run -- examples_ir2/break.tt
cargo run -- examples_ir2/nested_loop.tt
```

The compiler will print the generated IR and then execute it using the interpreter.

### Run Tests

```bash
cargo test
```

---

## Example

**Input (`loop.tt`):**
```
func main() {
    int i;
    i = 0;
    while i < 10 {
        print(i);
        i = i + 1;
    }
}
```

**Generated IR:**
```
%func main()
%int i
%mov i, 0
:loopbegin1
%int _temp1
%lt _temp1, i, 10
%branch_ifn _temp1, :endloop1
%out i
%int _temp2
%add _temp2, i, 1
%mov i, _temp2
%jmp :loopbegin1
:endloop1
%endfunc
```

**Output:**
```
0
1
2
3
4
5
6
7
8
9
```

---

## Semantic Error Checking

The compiler detects the following errors at compile time and halts without emitting any IR:

- Use of an undeclared variable
- Use of a scalar variable as an array, or vice versa
- Call to an undefined function
- `break` or `continue` used outside of a loop

Example error output:
```
Parser Error: Variable 'x' used without declaration
Parser Error: break statement is outside a loop
```

---

## Known Limitations

- Error messages do not include line numbers or column positions
- The IR interpreter (`interpreter.rs`) was provided by the course and is not original work

---

## Acknowledgements

Completed as part of **CS152: Compiler Design** at the University of California, Riverside.

Course staff provided the project specifications, the grammar for the Teh Tarik language, example test programs, and the IR interpreter (`interpreter.rs`). All compiler code — the lexer, parser, symbol table, and code generator — is original work.
