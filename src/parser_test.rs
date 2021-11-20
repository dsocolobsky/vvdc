use super::*;
use crate::{parser::ExpressionType, tokens::Token};
use lexer::lex_program;
use parser::parse;

#[test]
fn literals() {
    let tokens = lex_program(r#"1337 "banana" tomato"#);
    let expressions = parse(&tokens);

    assert_eq!(3, expressions.len());
    assert!(expressions
        .iter()
        .all(|expression| expression.expression_type == ExpressionType::LiteralExpression));

    assert_eq!(Token::number(1337), expressions[0].token);
    assert_eq!(Token::string("banana"), expressions[1].token);
    assert_eq!(Token::literal("tomato"), expressions[2].token);
}

#[test]
fn unary_negation() {
    let tokens = lex_program("!5");
    let expressions = parse(&tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(
        ExpressionType::PrefixExpression,
        expressions[0].expression_type
    );
    assert_eq!(Token::bang(), expressions[0].token, "expression is a bang");

    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::LiteralExpression,
        right_expression.expression_type
    );
    assert_eq!(
        Token::number(5),
        right_expression.token,
        "right expression is number 5"
    );
}

#[test]
fn return_number() {
    let tokens = lex_program("return 42;");
    let expressions = parse(&tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(
        ExpressionType::ReturnExpression,
        expressions[0].expression_type
    );
    assert_eq!(
        Token::keyword_return(),
        expressions[0].token,
        "return token is 'return'"
    );

    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::LiteralExpression,
        right_expression.expression_type
    );
    assert_eq!(
        Token::number(42),
        right_expression.token,
        "right expression is number 42"
    );
}

#[test]
fn return_expression() {
    let tokens = lex_program("return !1;");
    let expressions = parse(&tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(
        ExpressionType::ReturnExpression,
        expressions[0].expression_type
    );
    assert_eq!(
        Token::keyword_return(),
        expressions[0].token,
        "return token is 'return'"
    );

    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::PrefixExpression,
        right_expression.expression_type
    );
    assert_eq!(
        Token::bang(),
        right_expression.token,
        "right expression token is a bang"
    );

    // Right side of negation expression => number 1
    let negation_expression_right = right_expression.right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::LiteralExpression,
        negation_expression_right.expression_type
    );
    assert_eq!(
        Token::number(1),
        negation_expression_right.token,
        "right side of negation expression is number 1"
    );
}

#[test]
fn return_negation_of_negation() {
    let tokens = lex_program("return !!5;");
    let expressions = parse(&tokens);
    
    // return (!(!5))
    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(
        ExpressionType::ReturnExpression,
        expressions[0].expression_type,
        "outer expression is a return"
    );

    // !(!5)
    let right_expression = expressions[0].right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::PrefixExpression,
        right_expression.expression_type,
        "first prefix expression is a prefix"
    );
    assert_eq!(
        Token::bang(),
        right_expression.token,
        "right expression token is a bang"
    );

    // !5
    let right_right_expression = right_expression.right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::PrefixExpression,
        right_right_expression.expression_type,
        "second prefix expression is a prefix"
    );
    assert_eq!(
        Token::bang(),
        right_right_expression.token,
        "right right expression token is a bang"
    );

    let number_exp = right_right_expression.right.as_ref().unwrap();
    assert_eq!(
        ExpressionType::LiteralExpression,
        number_exp.expression_type,
        "inner expression is a literal"
    );
    assert_eq!(
        Token::number(5),
        number_exp.token,
        "negated number is 5"
    );
}
