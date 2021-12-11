use super::*;
use crate::{parser::ExpressionType, tokens::Token, tokens::TokenType};
use lexer::lex_program;
use parser::parse;

macro_rules! expect_prefix {
    ($toktype:expr, $seen:expr) => {
        assert_eq!(ExpressionType::PrefixExpression, $seen.expression_type, "expected a prefix expression");
        assert_eq!($toktype, $seen.token.token_type);
    };
    ($seen:expr) => {
        assert_eq!(ExpressionType::PrefixExpression, $seen.expression_type, "expected a prefix expression")
    };
}

macro_rules! expect_literal_type {
    ($seen:expr) => {
        assert_eq!(ExpressionType::LiteralExpression, $seen.expression_type, "expected a literal expression");
    }
}

macro_rules! expect_number {
    ($number:expr, $seen:expr) => {
        expect_literal_type!($seen);
        assert_eq!(Token::number($number), $seen.token, "wrong number value");
    }
}

macro_rules! expect_literal {
    ($lit:expr, $seen:expr) => {
        expect_literal_type!($seen);
        assert_eq!(Token::literal($lit), $seen.token, "wrong literal value");
    }
}

macro_rules! expect_string {
    ($str:expr, $seen:expr) => {
        expect_literal_type!($seen);
        assert_eq!(Token::string($str), $seen.token, "wrong string value");
    }
}

macro_rules! expect_return {
    ($a:expr) => {
        assert_eq!(ExpressionType::ReturnExpression, $a.expression_type);
        assert_eq!(TokenType::KeywordReturn, $a.token.token_type, "return token is not 'return'");
    }
}

#[test]
fn literals() {
    let tokens = lex_program(r#"1337 "banana" tomato;"#);
    let expressions = parse(tokens);

    assert_eq!(3, expressions.len());
    expect_number!(1337, expressions[0]);
    expect_string!("banana", expressions[1]);
    expect_literal!("tomato", expressions[2]);
}

#[test]
fn unary_negation() {
    let tokens = lex_program("!5;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_prefix!(TokenType::Bang, expressions[0]);

    let right_expression = expressions[0].right_side();
    expect_number!(5, right_expression);
}

#[test]
fn double_negation() {
    let tokens = lex_program("!!5;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_prefix!(TokenType::Bang, expressions[0]);

    let right_expression = expressions[0].right_side();
    expect_prefix!(TokenType::Bang, right_expression);

    let numeric_expression = right_expression.right_side();
    expect_number!(5, numeric_expression);
}

#[test]
fn return_number() {
    let tokens = lex_program("return 42;");
    let expressions = parse(tokens);

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

    let right_expression = expressions[0].right_side();
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
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);
    let right_expression = expressions[0].right_side();
    expect_prefix!(TokenType::Bang, right_expression);
    // Right side of negation expression => number 1
    let negation_expression_right = right_expression.right_side();
    expect_number!(1, negation_expression_right);
}

#[test]
fn return_negation_of_negation() {
    let tokens = lex_program("return !!5;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);
    let not_not_five = expressions[0].right_side();
    expect_prefix!(TokenType::Bang, not_not_five);
    let not_five = not_not_five.right_side();
    expect_prefix!(TokenType::Bang, not_five);
    let five = not_five.right_side();
    expect_number!(5, five);
}

#[test]
fn addition_of_two_numbers() {
    let tokens = lex_program("12 + 4;");
    let expressions = parse(tokens);
    
    // return (!(!5))
    assert_eq!(1, expressions.len(), "number of expressions");
    assert_eq!(
        ExpressionType::InfixExpression,
        expressions[0].expression_type,
        "outer expression is an addition"
    );

    let left_expression = expressions[0].left_side();
    let right_expression = expressions[0].right_side();
    expect_number!(12, left_expression);
    expect_number!(4, right_expression);
}

/*#[test]
fn return_addition_of_two_numbers() {
    let tokens = lex_program("return 12 + 4;");
    let expressions = parse(tokens);
    
    // return (!(!5))
    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);

    let infix_expression = expressions[0].right_side();
    assert_eq!(
        ExpressionType::InfixExpression,
        infix_expression.expression_type,
        "inner expression is an addition"
    );

    let left_expression = infix_expression.left_side();
    let right_expression = infix_expression.right_side();
    expect_number!(12, left_expression);
    expect_number!(4, right_expression);

    let right_expression = infix_expression.right_side();
    assert_eq!(
        ExpressionType::LiteralExpression,
        right_expression.expression_type,
        "rhs is literal"
    );
    assert_eq!(
        Token::number(4),
        right_expression.token,
        "rhs token is literal 4"
    );
}*/