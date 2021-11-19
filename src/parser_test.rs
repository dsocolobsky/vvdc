use super::*;
use lexer::lex_program;
use parser::parse;
use crate::{parser::ExpressionType, tokens::Token};

#[test]
fn literals() {
    let tokens = lex_program(r#"1337 "banana" tomato"#);
    let expressions  = parse(&tokens);

    assert_eq!(3, expressions.len());
    assert!(expressions.iter().all(|expression|
        expression.expression_type == ExpressionType::LiteralExpression
    ));

    assert_eq!(Token::number(1337), expressions[0].token);
    assert_eq!(Token::string("banana"), expressions[1].token);
    assert_eq!(Token::literal("tomato"), expressions[2].token);
}

#[test]
fn unary_negation() {
    let tokens = lex_program("!5");
    let expressions  = parse(&tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(ExpressionType::PrefixExpression, expressions[0].expression_type);
    assert_eq!(Token::bang(), expressions[0].token, "expression is a bang");

    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(ExpressionType::LiteralExpression, right_expression.expression_type);
    assert_eq!(Token::number(5), right_expression.token, "right expression is number 5");
}

#[test]
fn return_number() {
    let tokens = lex_program("return 42;");
    let expressions  = parse(&tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(ExpressionType::ReturnExpression, expressions[0].expression_type);
    assert_eq!(Token::keyword_return(), expressions[0].token, "return token is 'return'");

    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(ExpressionType::LiteralExpression, right_expression.expression_type);
    assert_eq!(Token::number(42), right_expression.token, "right expression is number 42");
}