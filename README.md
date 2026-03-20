# Compiler design project

By Luke Matsunaga

Project completed as part of a compilers course. 

## Objectives  
- Understand the core phases of compiler construction  
- Implement a working compiler using **Rust**  
- Design a simple, structured programming language  
- Explore parsing strategies and code generation techniques

## Structure  

The project is divided into multiple phases, each corresponding to a key stage in compiler development:

### Phase 0: Foundations  
- Introduction to Rust  
- Language design and specification  

### Phase 1: Lexer  
- Tokenization of source code  
- Handling keywords, identifiers, literals, and operators  

### Phase 2: Parser  
- Syntax analysis  
- Construction of abstract syntax trees (AST)  

### Phase 3: Simple Code Generation  
- Translating AST into intermediate or target code  
- Basic instruction generation  

### Phase 4: Complex Code Generation  
- Advanced constructs (e.g., control flow, loops)  
- Optimization and structured output


## Teh Tarik Programming Language

This programming language is named after Teh Tarik, which is the national drink of Malaysia.


## Documentation  

Detailed documentation for each phase can be found below:

- [Phase 0: Introduction to Rust](docs/TehTarik.md)

- [Phase 0: Language Specification](docs/phase0.md)

- [Phase 1: Building a Lexer](docs/phase1.md)

- [Phase 2: Building a Parser](docs/phase2.md)

- [Phase 3: Simple Code Generation](docs/phase3.md)

- [Phase 4: Complex Code Generation](docs/phase4.md)

## Getting Started  

### Prerequisites  
- Install Rust: https://www.rust-lang.org/tools/install  
- Cargo (comes with Rust)

### Build the Project  
```bash
cargo build
```

### Run the Compiler 
```bash
cd src/
cargo run -- <examples_dir>/<.tt file>
```
Example command:
```bash
cargo run -- examples_parser/array.tt
```

## Acknowledgements

This project was completed as part of the course **CS152: Compiler Design** at the University of California, Riverside.

I would like to thank the course instructor and teaching staff for providing:
- The project specifications and grammar for the Teh Tarik programming language
- Example test cases and guidance throughout each phase
- The interpreter (`interpreter.rs`) used for executing the generated IR

Course materials, including lecture notes and assignment descriptions, were used as references 
for implementing the lexer, parser, and code generation phases of this project.

All code written for this project is our own, except for any provided starter code or materials 
explicitly given as part of the assignment.
