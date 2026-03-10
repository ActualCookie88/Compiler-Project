use crate::token::Token;
use crate::parser::function::parse_function;
use crate::parser::statement::CodeGenState;

// testing if push works comment

#[derive(Clone)]
pub struct Var {
    pub name: String,
    pub is_array: bool,
    pub size: i32, // 0 if scalar
}

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub params: Vec<Var>,
    pub locals: Vec<Var>,
}

pub struct SymbolTable {
    pub functions: Vec<Func>,
}

// helpers
pub fn find_function<'a>(table: &'a SymbolTable, name: &str) -> Option<&'a Func> {
    table.functions.iter().find(|f| f.name == name)
}

pub fn find_function_mut<'a>(table: &'a mut SymbolTable, name: &str) -> Option<&'a mut Func> {
    table.functions.iter_mut().find(|f| f.name == name)
}

pub fn find_variable<'a>(func: &'a Func, name: &str) -> Option<&'a Var> {
    for p in &func.params {
        if p.name == name {
            return Some(p);
        }
    }

    for v in &func.locals {
        if v.name == name {
            return Some(v);
        }
    }

    None
}

pub fn variable_exists(func: &Func, name: &str) -> bool {
    find_variable(func, name).is_some()
}

// insert helpers (to build table)
pub fn add_function(table: &mut SymbolTable, name: String) -> Result<(), String> {
    if find_function(table, &name).is_some() {
        return Err(format!("Function '{}' defined more than once", name));
    }

    table.functions.push(Func {
        name,
        params: Vec::new(),
        locals: Vec::new(),
    });

    Ok(())
}

pub fn add_param(table: &mut SymbolTable, func_name: &str, var: Var) -> Result<(), String> {
    // SEMANTIC CHECK: "Creating an array of size <= 0"
    if var.is_array && var.size <= 0 {
        return Err(format!("Array '{}' must have size > 0", var.name));
    }

    let func = find_function_mut(table, func_name)
        .ok_or_else(|| format!("Function '{}' not found", func_name))?;

    // SEMANTIC CHECK: Defining a variable more than once
    if variable_exists(func, &var.name) {
        return Err(format!("Variable '{}' defined more than once", var.name));
    }

    func.params.push(var);
    Ok(())
}

pub fn add_local(table: &mut SymbolTable, func_name: &str, var: Var) -> Result<(), String> {
    // SEMANTIC CHECK: "Creating an array of size <= 0"
    if var.is_array && var.size <= 0 {
        return Err(format!("Array '{}' must have size > 0", var.name));
    }
    
    let func = find_function_mut(table, func_name)
        .ok_or_else(|| format!("Function '{}' not found", func_name))?;

    // SEMANTIC CHECK: Defining a variable more than once
    if variable_exists(func, &var.name) {
        return Err(format!("Variable '{}' defined more than once", var.name));
    }

    func.locals.push(var);
    Ok(())
}
// parse programs with multiple functions
// loop over everything, outputting generated code.
pub fn parse_program(tokens: &Vec<Token>, 
                        index: &mut usize, 
                        state: &mut CodeGenState
                    ) -> Result<String, String> {
    assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));

    let mut code = String::new();
    let mut table = SymbolTable { functions: Vec::new() };
    let mut state = CodeGenState { label_counter: 0 };
    while !at_end(tokens, *index) {
        match parse_function(tokens, index, &mut table, &mut state) {
            Ok(function_code) => {
                code += &function_code;
            }
            Err(e) => { return Err(e); }
        }
    }

    // SEMANTIC CHECK: "Not defining a main function"
    if find_function(&table, "main").is_none() {
        return Err(String::from("No 'main' function defined"));
    }
    return Ok(code);
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
    match tokens[index] {
    Token::End => { true }
    _ => { false }
    }
}