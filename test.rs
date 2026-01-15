use std::fs;

fn main() {
    // mutable variable declaration
    let mut var = 0;
    while var < 3 {
        // mutate the 'var'
        println!("var = {}", var);
        var += 1;
    }

    // block expressions
    let v = {
        let mut num = 0;
        while num < 5 {
            num += 1;
        }
        num 
    };

    println!("v = {}", v);

    // Strings and &str
    let num1 = 50;
    let num2 = 100;
    let s: String = format!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{}", s);

    let name = "Ada Lovelace";
    let first_name = &name[0..3];
    let last_name =  &name[4..];
    println!("{first_name}");
    println!("{last_name}");

    if name.starts_with("Ada") {
    println!("Your name starts with Ada");
    } else {
        println!("Your name starts with something else");
    }

    // matching
    let animal = "cat";
    match animal {
        "cow" => {
            println!("cow says: \"Moo!\"");
        }
        "cat" => {
            println!("cat says: \"Meow!\"");
        }
        "dog" => {
            println!("dog says: \"Wuff!\"");
        }
        _ => {
            println!("default case = {}", animal);
        }
    }

    // structs
    /*
    let vec3 = Vec3 {
        x : 1.0,
        y : 1.0,
        z : 1.0,
    };
    */

    // enum
    #[derive(Debug, Clone)]
        enum Token {
        Plus,
        Subtract,
        Multiply,
        Divide,
        Modulus,
        Assign,
        Num(i32),
    }

    // Option type
    let option: Option<i32> = Some(1);
    if let Some(value) = option {
        // value has been unwrapped. use it.
        println!("value = {}", value);
    }
    match option {
        Some(value) => {
            // value has been unwrapped. use it.
            println!("value = {}", value);
        }
        None => {}
    }

    // Files
    let filename = "file.txt";
    let code = match fs::read_to_string(filename) {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => { // Ok(val) == Some(val)
            code
        } 
    };

    println!("Code:");
    println!("{}", code);
}
