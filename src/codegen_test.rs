use super::*;
use codegen::generate_code;
use lexer::lex_program;
use parser::parse;

#[test]
fn return_a_literal() {
    let tokens = lex_program("return 13;");
    let expressions = parse(&tokens);
    let code = generate_code(expressions);

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
