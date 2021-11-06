pub mod tokens;
pub mod lexer;
use std::fs;

fn main() {
    let code = fs::read_to_string("test.dl").expect("Unable to read file");
    println!("{}", code);
    println!("================================");
    let tokens = lexer::lex_program(&code);
    println!("{:?}", &tokens);
    println!("\n");
    println!("\n");
}

mod test {
    #[test]
    fn main_test() {
        
    }
}

#[cfg(test)]
mod lexer_test;