pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod tokens;
use std::fs;
use std::process::{Command, ExitStatus};

fn main() {
    let code = fs::read_to_string("programs/2_return_addition.vvdl").expect("Unable to read file");
    println!("{}", code);
    println!("============ TOKENS ============");
    let tokens = lexer::lex_program(&code);
    println!("{:?}", tokens);
    println!("============ AST ============");
    let expressions = parser::parse(tokens);
    println!("{:?}", &expressions);
    println!("============ ASSEMBLY ============");
    let code = codegen::generate_code(expressions);
    println!("{}", code);

    println!("============ BUILDING ============");
    // Write .asm file
    fs::write("programs/obj/output.asm", code).expect("Unable to write file");

    // Call nasm on .asm file to generate .o file
    let nasm_output = Command::new("nasm")
        .args(["-f elf64", "programs/obj/output.asm"])
        .status()
        .expect("failed to execute nasm");
    if nasm_output.success() {
        println!("nasm ok")
    } else {
        println!("nasm error: {}", nasm_output);
    }

    // Call linker to generate final executable
    let ld_output = Command::new("ld")
        .args(["-s", "-o", "programs/obj/output", "programs/obj/output.o"])
        .status()
        .expect("failed to execute ld");
    if ld_output.success() {
        println!("ld ok")
    } else {
        println!("ld error: {}", ld_output);
    }
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
