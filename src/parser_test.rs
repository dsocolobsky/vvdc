use super::*;
use lexer::lex_program;
use parser::parse;
use crate::tokens::Token;

#[test]
fn unary_negation() {
    let tokens = lex_program("!5");
    let expressions  = parse(&tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(Token::bang(), expressions[0].token, "expression is a bang");

    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(Token::number(5), right_expression.token, "right expression is number 5");
}