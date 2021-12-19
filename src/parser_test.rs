use super::*;
use crate::{tokens::TokenType, parser::ExpressionType};
use lexer::lex_program;
use parser::parse;

macro_rules! expect_prefix {
    ($toktype:expr, $seen:expr) => {
        assert_eq!(ExpressionType::PrefixExpression, $seen.get_type(), "expected a prefix expression");
        assert_eq!($toktype, $seen.token.token_type);
    }
}

macro_rules! expect_infix {
    ($toktype:expr, $seen:expr) => {
        assert_eq!(ExpressionType::InfixExpression, $seen.get_type(), "expected an infix expression");
        assert_eq!($toktype, $seen.token.token_type);
    };
}

macro_rules! expect_number {
    ($number:expr, $seen:expr) => {
        assert_eq!(ExpressionType::Number, $seen.get_type(), "expected a number");
        assert_eq!(format!("{}", $number), *$seen.to_string(), "wrong number value");
    }
}

macro_rules! expect_identifier {
    ($lit:expr, $seen:expr) => {
        assert_eq!(ExpressionType::Identifier, $seen.get_type(), "expected an infix expression");
        assert_eq!($lit, $seen.to_string(), "wrong literal value");
    }
}

macro_rules! expect_string {
    ($str:expr, $seen:expr) => {
        assert_eq!(ExpressionType::String, $seen.get_type(), "expected a string");
        assert_eq!($str, $seen.to_string(), "wrong string value");
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
    //expect_string!("banana", expressions[1]);
    //expect_identifier!("tomato", expressions[2]);
}

/*#[test]
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
    expect_return!(expressions[0]);

    let number_42 = expressions[0].right_side();
    expect_number!(42, number_42);
}

#[test]
fn return_expression() {
    let tokens = lex_program("return !1;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);
    let not_one = expressions[0].right_side();
    expect_prefix!(TokenType::Bang, not_one);
    let number_one = not_one.right_side();
    expect_number!(1, number_one);
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

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_infix!(TokenType::Plus, expressions[0]);
    let left_expression = expressions[0].left_side();
    let right_expression = expressions[0].right_side();
    expect_number!(12, left_expression);
    expect_number!(4, right_expression);
}

#[test]
fn return_addition_of_two_numbers() {
    let tokens = lex_program("return 12 + 4;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);

    let addition = expressions[0].right_side();
    expect_infix!(TokenType::Plus, addition);

    let left_expression = addition.left_side();
    let right_expression = addition.right_side();
    expect_number!(12, left_expression);
    expect_number!(4, right_expression);
}

#[test]
fn return_addition_of_three_numbers() {
    let tokens = lex_program("return 12 + 4 + 6;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);

    let addition_12_rest = expressions[0].right_side();
    expect_infix!(TokenType::Plus, addition_12_rest);
    expect_number!(12, addition_12_rest.left_side());

    let addition_4_6 = addition_12_rest.right_side();
    expect_infix!(TokenType::Plus, addition_4_6);
    expect_number!(4, addition_4_6.left_side());
    expect_number!(6, addition_4_6.right_side());
}

#[test]
fn return_addition_of_four_numbers() {
    let tokens = lex_program("return 12 + 4 + 6 + 3;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_return!(expressions[0]);

    let addition_12_rest = expressions[0].right_side();
    expect_number!(12, addition_12_rest.left_side());

    let addition_4_rest = addition_12_rest.right_side();
    expect_number!(4, addition_4_rest.left_side());

    let addition_6_3 = addition_4_rest.right_side();
    expect_number!(6, addition_6_3.left_side());
    expect_number!(3, addition_6_3.right_side());
}

#[test]
fn subtraction_of_two_numbers() {
    let tokens = lex_program("12 - 4;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");
    expect_infix!(TokenType::Minus, expressions[0]);
    let left_expression = expressions[0].left_side();
    let right_expression = expressions[0].right_side();
    expect_number!(12, left_expression);
    expect_number!(4, right_expression);
}

#[test]
fn subtraction_of_four_numbers() {
    let tokens = lex_program("10 - 2 - 4 - 1;");
    let expressions = parse(tokens);

    assert_eq!(1, expressions.len(), "number of expressions");

    let subtraction_10_rest = &expressions[0];
    expect_infix!(TokenType::Minus, subtraction_10_rest);
    expect_number!(10, subtraction_10_rest.left_side());

    let subtraction_2_rest = subtraction_10_rest.right;
    expect_infix!(TokenType::Minus, subtraction_2_rest);
    expect_number!(2, *subtraction_2_rest.left);

    let subtraction_4_1 = *subtraction_2_rest.right;
    expect_infix!(TokenType::Minus, subtraction_4_1);
    expect_number!(4, *subtraction_4_1.left);
    expect_number!(1, *subtraction_4_1.right);
}
*/