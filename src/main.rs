pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod tokens;
use std::{env, fs};
use std::path::Path;
use std::process::{Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "programs/test.vvdl".to_string()
    };
    let program_name = Path::new(&program_path).file_stem().unwrap().to_str().unwrap();
    println!("Compiling {}", program_path);
    println!("Program name: {}", program_name);

    let code = fs::read_to_string(program_path.clone()).expect("Unable to read file");
    println!("============ CODE ============");
    println!("{}", code);
    println!("============ TOKENS ============");
    let tokens = lexer::lex_program(&code);
    println!("{:?}", tokens);
    println!("============ AST ============");
    let expressions = parser::parse(tokens);
    //println!("{:?}", &expressions);
    println!("============ ASSEMBLY ============");
    let code = codegen::generate_code(expressions);
    println!("{}", code);

    println!("============ BUILDING ============");
    // Write .asm file
    let asm_path = format!("programs/obj/{}.asm", program_name);
    fs::write(asm_path.clone(), code).expect("Unable to write file");

    // Call nasm on .asm file to generate .o file
    let nasm_output = Command::new("nasm")
        .args(["-f elf64", &*asm_path])
        .status()
        .expect("failed to execute nasm");
    if nasm_output.success() {
        println!("nasm ok")
    } else {
        println!("nasm error: {}", nasm_output);
    }

    // Call linker to generate final executable
    let executable_path = format!("programs/obj/{}", program_name);
    let obj_path = format!("{}.o", executable_path);
    let ld_output = Command::new("ld")
        .args(["-s", "-o", &*executable_path, &*obj_path])
        .status()
        .expect("failed to execute ld");
    if ld_output.success() {
        println!("ld ok")
    } else {
        println!("ld error: {}", ld_output);
    }

    println!("============ RUNNING ============");
    println!("running: {}", executable_path);
    let obj_output = Command::new(format!("./{}", executable_path))
        .status()
        .expect("failed to run compiled program");
    println!("output: {}", obj_output);
}

mod test {
    #[test]
    fn main_test() {}
}

#[cfg(test)]
mod lexer_test;

#[cfg(test)]
mod parser_test;

#[cfg(test)]
mod codegen_test;
