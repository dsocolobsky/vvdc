use super::*;
use codegen::generate_code;
use lexer::lex_program;
use parser::parse;

/*
#[test]
fn return_a_literal() {
    let tokens = lex_program("return 13;");
    let expressions = parse(tokens);
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

#[test]
fn return_a_negated_positive_number() {
    let tokens = lex_program("return !5;");
    let expressions = parse(tokens);
    let code = generate_code(expressions);

    assert_eq!(
        r#"section .text
global _start
_start:
mov rbx, 0
mov rax, 1
int 0x80
"#,
        code
    );
}

#[test]
fn return_a_negated_zero() {
    let tokens = lex_program("return !0;");
    let expressions = parse(tokens);
    let code = generate_code(expressions);

    assert_eq!(
        r#"section .text
global _start
_start:
mov rbx, 1
mov rax, 1
int 0x80
"#,
        code
    );
}

#[test]
fn return_negation_of_a_negation_of_a_literal() {
    let tokens = lex_program("return !!5;");
    let expressions = parse(tokens);
    let code = generate_code(expressions);

    assert_eq!(
        r#"section .text
global _start
_start:
mov rbx, 0
xor rbx, 1
mov rax, 1
int 0x80
"#,
        code
    );
}

#[test]
fn return_addition_of_two_numbers() {
    let tokens = lex_program("return 12 + 4;");
    let expressions = parse(tokens);
    let code = generate_code(expressions);

    assert_eq!(
        r#"section .text
global _start
_start:
mov rbx, 12
add rbx, 4
mov rax, 1
int 0x80
"#,
        code
    );
}

#[test]
fn return_addition_of_three_numbers() {
    let tokens = lex_program("return 12 + 4 + 6;");
    let expressions = parse(tokens);
    let code = generate_code(expressions);

    assert_eq!(
        r#"section .text
global _start
_start:
mov rbx, 12
add rbx, 4
add rbx, 6
mov rax, 1
int 0x80
"#,
        code
    );
}

#[test]
fn return_addition_of_four_numbers() {
    let tokens = lex_program("return 12 + 4 + 6 + 3;");
    let expressions = parse(tokens);
    let code = generate_code(expressions);

    assert_eq!(
        r#"section .text
global _start
_start:
mov rbx, 12
add rbx, 4
add rbx, 6
add rbx, 3
mov rax, 1
int 0x80
"#,
        code
    );
}

 */