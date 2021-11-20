use super::*;
use crate::tokens::Token;
use crate::tokens::TokenType;
use lexer::lex_program;

#[test]
fn empty_string_produces_empty_list_of_tokens() {
    let tokens = lex_program("");
    assert_eq!(0, tokens.len());
}

#[test]
fn token_assignment() {
    let tokens = lex_program("=");
    assert_eq!([Token::assignment()], &tokens[..]);
}

#[test]
fn token_plus() {
    let tokens = lex_program("+");
    assert_eq!([Token::plus()], &tokens[..]);
}

#[test]
fn token_minus() {
    let tokens = lex_program("-");
    assert_eq!([Token::minus()], &tokens[..]);
}

#[test]
fn token_asterisk() {
    let tokens = lex_program("*");
    assert_eq!([Token::asterisk()], &tokens[..]);
}

#[test]
fn token_equals() {
    let tokens = lex_program("==");
    assert_eq!([Token::equals()], &tokens[..]);
}

#[test]
#[should_panic]
fn invalid_token_should_raise_error() {
    let tokens = lex_program("=%");
    assert_eq!(0, tokens.len());
}

#[test]
fn several_tokens() {
    let tokens = lex_program("=+-*+=;");
    assert_eq!(
        [
            Token::assignment(),
            Token::plus(),
            Token::minus(),
            Token::asterisk(),
            Token::plus(),
            Token::assignment(),
            Token::semicolon()
        ],
        &tokens[..]
    );
}

#[test]
fn several_tokens_with_spaces() {
    let tokens = lex_program(" + -    ;; *");
    assert_eq!(
        [
            Token::plus(),
            Token::minus(),
            Token::semicolon(),
            Token::semicolon(),
            Token::asterisk()
        ],
        &tokens[..]
    );
}

#[test]
fn literal() {
    let tokens = lex_program("banana");
    assert_eq!([Token::literal("banana")], &tokens[..]);
}

#[test]
fn single_letter_literal() {
    let tokens = lex_program("a");
    assert_eq!([Token::literal("a")], &tokens[..]);
}

#[test]
fn literal_and_token() {
    let tokens = lex_program("mango=");
    assert_eq!([Token::literal("mango"), Token::assignment()], &tokens[..]);
}

#[test]
fn two_literals() {
    let tokens = lex_program("t omate");
    assert_eq!([Token::literal("t"), Token::literal("omate")], &tokens[..]);
}

#[test]
fn literal_number_and_string() {
    let tokens = lex_program(r#" banana 1337 "kiwi" "#);
    assert_eq!(
        [
            Token::literal("banana"),
            Token::number(1337),
            Token::string("kiwi")
        ],
        &tokens[..]
    );
}

#[test]
fn complex_expression() {
    let tokens = lex_program("radio = pi*e;");
    assert_eq!(
        [
            Token::literal("radio"),
            Token::assignment(),
            Token::literal("pi"),
            Token::asterisk(),
            Token::literal("e"),
            Token::semicolon()
        ],
        &tokens[..]
    );
}

#[test]
fn complex_expression_with_equality() {
    let tokens = lex_program(r#"8==ocho == "ocho""#);
    assert_eq!(
        [
            Token::number(8),
            Token::equals(),
            Token::literal("ocho"),
            Token::equals(),
            Token::string("ocho"),
        ],
        &tokens[..]
    );
}

#[test]
fn a_single_digit_number() {
    let tokens = lex_program("4");
    assert_eq!([Token::number(4)], &tokens[..]);
}

#[test]
fn a_multiple_digit_number() {
    let tokens = lex_program("1337");
    assert_eq!([Token::number(1337)], &tokens[..]);
}

#[test]
fn number_and_symbol() {
    let tokens = lex_program("4*32");
    assert_eq!(
        [Token::number(4), Token::asterisk(), Token::number(32)],
        &tokens[..]
    );
}

#[test]
fn symbol_and_number() {
    let tokens = lex_program("=17");
    assert_eq!([Token::assignment(), Token::number(17)], &tokens[..]);
}

#[test]
fn complex_expression_with_numbers() {
    let tokens = lex_program("x = 4 * 35+2 1;");
    assert_eq!(
        [
            Token::literal("x"),
            Token::assignment(),
            Token::number(4),
            Token::asterisk(),
            Token::number(35),
            Token::plus(),
            Token::number(2),
            Token::number(1),
            Token::semicolon()
        ],
        &tokens[..]
    );
}

#[test]
fn single_letter_as_string() {
    let tokens = lex_program(r#""f""#);
    assert_eq!([Token::string("f")], &tokens[..]);
}

#[test]
fn single_string() {
    let tokens = lex_program(r#""cafe""#);
    assert_eq!([Token::string("cafe")], &tokens[..]);
}

#[test]
fn string_with_spaces() {
    let tokens = lex_program(r#""canada nice country""#);
    assert_eq!([Token::string("canada nice country")], &tokens[..]);
}

#[test]
fn comparison_operators() {
    let tokens = lex_program("4 < 19>2 >= !1 <= 1!=10");
    assert_eq!(
        [
            Token::number(4),
            Token::lt(),
            Token::number(19),
            Token::gt(),
            Token::number(2),
            Token::gteq(),
            Token::bang(),
            Token::number(1),
            Token::lteq(),
            Token::number(1),
            Token::unequal(),
            Token::number(10)
        ],
        &tokens[..]
    );
}

#[test]
fn parentheses() {
    let tokens = lex_program("(2) == !((4))");
    assert_eq!(
        [
            Token::lparen(),
            Token::number(2),
            Token::rparen(),
            Token::equals(),
            Token::bang(),
            Token::lparen(),
            Token::lparen(),
            Token::number(4),
            Token::rparen(),
            Token::rparen()
        ],
        &tokens[..]
    );
}

#[test]
fn braces() {
    let tokens = lex_program("abc{{x}}cba");
    assert_eq!(
        [
            Token::literal("abc"),
            Token::lbrace(),
            Token::lbrace(),
            Token::literal("x"),
            Token::rbrace(),
            Token::rbrace(),
            Token::literal("cba")
        ],
        &tokens[..]
    );
}

#[test]
fn newlines() {
    let tokens = lex_program("3*\n2 + \n 3;");
    assert_eq!(
        [
            Token::number(3),
            Token::asterisk(),
            Token::number(2),
            Token::plus(),
            Token::number(3),
            Token::semicolon()
        ],
        &tokens[..]
    );
}

#[test]
fn keywords() {
    let tokens: Vec<Token> = lex_program("if print while return let fn");
    let tokens: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        [
            TokenType::KeywordIf,
            TokenType::KeywordPrint,
            TokenType::KeywordWhile,
            TokenType::KeywordReturn,
            TokenType::KeywordLet,
            TokenType::KeywordFn
        ],
        &tokens[..]
    );
}

#[test]
fn sample_program() {
    let tokens = lex_program(
        r#"
            fn square(x) {
                return x * x;
            }

            fn main() {
                let y = square(4);
                while y > 12 {
                    print("answer is " y);
                }
                return 0;
            }
            "#,
    );

    assert_eq!(
        [
            Token::keyword_fn(),
            Token::literal("square"),
            Token::lparen(),
            Token::literal("x"),
            Token::rparen(),
            Token::lbrace(),
            Token::keyword_return(),
            Token::literal("x"),
            Token::asterisk(),
            Token::literal("x"),
            Token::semicolon(),
            Token::rbrace(),
            Token::keyword_fn(),
            Token::literal("main"),
            Token::lparen(),
            Token::rparen(),
            Token::lbrace(),
            Token::keyword_let(),
            Token::literal("y"),
            Token::assignment(),
            Token::literal("square"),
            Token::lparen(),
            Token::number(4),
            Token::rparen(),
            Token::semicolon(),
            Token::keyword_while(),
            Token::literal("y"),
            Token::gt(),
            Token::number(12),
            Token::lbrace(),
            Token::keyword_print(),
            Token::lparen(),
            Token::string("answer is "),
            Token::literal("y"),
            Token::rparen(),
            Token::semicolon(),
            Token::rbrace(),
            Token::keyword_return(),
            Token::number(0),
            Token::semicolon(),
            Token::rbrace()
        ],
        &tokens[..]
    );
}
