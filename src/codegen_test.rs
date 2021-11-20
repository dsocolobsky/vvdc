use super::*;
use lexer::lex_program;
use parser::parse;
use codegen::generate_code;

#[test]
fn return_a_literal() {
    let tokens = lex_program("return 13;");
    let expressions  = parse(&tokens);
    let code = generate_code(&expressions);

    assert_eq!(
r#"section .text
global _start
_start:
mov rbx, 13
mov rax, 1
int 0x80
"#,
    code
    );
}