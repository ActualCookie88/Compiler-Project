use crate::lexer::token::Token;
use crate::parser::function::parse_function;

/// Describes whether a variable is a plain scalar or a fixed-size array.
#[derive(Clone)]
pub enum VarKind {
    Scalar,
    Array(usize), // size is always > 0 here, enforced by type
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub kind: VarKind,
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Variable>,
    pub locals: Vec<Variable>,
}

pub struct SymbolTable {
    pub functions: Vec<Function>,
}

#[derive(Clone)]
pub struct LoopInfo {
    pub begin: String,
    pub end: String,
}

pub struct CodeGenState {
    pub label_counter: i64,
    pub loop_stack: Vec<LoopInfo>,  // stack for active loops
}

impl CodeGenState {
    pub fn new() -> Self {
        CodeGenState {
            label_counter: 1,
            loop_stack: Vec::new(),
        }
    }
}

/* ////////////////////////////////////////////////////////////////////

Symbol Table Lookups

//////////////////////////////////////////////////////////////////// */

pub fn find_function<'a>(table: &'a SymbolTable, name: &str) -> Option<&'a Function> {
    table.functions.iter().find(|f| f.name == name)
}

pub fn find_function_mut<'a>(table: &'a mut SymbolTable, name: &str) -> Option<&'a mut Function> {
    table.functions.iter_mut().find(|f| f.name == name)
}

// Search params first, then locals. Returns the first match found.
pub fn find_variable<'a>(func: &'a Function, name: &str) -> Option<&'a Variable> {
    func.params
        .iter()
        .chain(func.locals.iter())
        .find(|v| v.name == name)
}

/* ////////////////////////////////////////////////////////////////////

Symbol Table Insertion

//////////////////////////////////////////////////////////////////// */

pub fn add_function(table: &mut SymbolTable, name: String) -> Result<(), String> {
    // SEMANTIC CHECK: function defined more than once
    if find_function(table, &name).is_some() {
        return Err(format!("Function '{}' defined more than once", name));
    }
 
    table.functions.push(Function {
        name,
        params: Vec::new(),
        locals: Vec::new(),
    });
 
    Ok(())
}

pub fn add_param(table: &mut SymbolTable, func_name: &str, var: Variable) -> Result<(), String> {
    let func = find_function_mut(table, func_name)
        .ok_or_else(|| format!("Function '{}' not found", func_name))?;

    add_variable_to(&mut func.params, &mut func.locals, var)
}
 
pub fn add_local(table: &mut SymbolTable, func_name: &str, var: Variable) -> Result<(), String> {
    let func = find_function_mut(table, func_name)
        .ok_or_else(|| format!("Function '{}' not found", func_name))?;

    add_variable_to(&mut func.locals, &mut func.params, var)
}

fn add_variable_to(target: &mut Vec<Variable>, other: &mut Vec<Variable>, var: Variable) -> Result<(), String> {
    // SEMANTIC CHECK: array size must be > 0
    if let VarKind::Array(size) = var.kind {
        if size == 0 {
            return Err(format!("Array '{}' must have size > 0", var.name));
        }
    }
 
    // SEMANTIC CHECK: variable defined more than once
    let already_exists = target.iter().chain(other.iter()).any(|v| v.name == var.name);
    if already_exists {
        return Err(format!("Variable '{}' defined more than once", var.name));
    }
 
    target.push(var);
    Ok(())
}

/* ////////////////////////////////////////////////////////////////////

Top-Level Parse Entry Point

//////////////////////////////////////////////////////////////////// */

// Parse a complete program (one or more function definitions) and return the generated assembly/IR as a single `String`
pub fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
    // Ensure the caller upholds the token-stream invariant.
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));

    let mut code = String::new();
    let mut table = SymbolTable { functions: Vec::new() };
    let mut state = CodeGenState::new();

    while !at_end(tokens, *index) {
        let function_code = parse_function(tokens, index, &mut table, &mut state)?;
        code.push_str(&function_code)
    }

    // SEMANTIC CHECK: a 'main' function must be defined
    if find_function(&table, "main").is_none() {
        return Err(String::from("No 'main' function defined"));
    }
    
    Ok(code)
}

// Helper
fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
    match tokens[index] {
    Token::End => { true }
    _ => { false }
    }
}