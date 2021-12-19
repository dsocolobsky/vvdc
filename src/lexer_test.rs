use super::*;
use crate::tokens::Token;
use crate::tokens::TokenType;
use lexer::lex_program;

fn tokens_to_literals(tokens: &Vec<Token>) -> Vec<String> {
    tokens.into_iter().map(|token| token.literal.clone()).collect()
}

#[test]
fn empty_string_produces_empty_list_of_tokens() {
    let tokens = lex_program("");
    assert_eq!(0, tokens.len());
}

#[test]
fn token_assignment() {
    let tokens = lex_program("=");
    assert_eq!(["="], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_plus() {
    let tokens = lex_program("+");
    assert_eq!(["+"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_minus() {
    let tokens = lex_program("-");
    assert_eq!(["-"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_asterisk() {
    let tokens = lex_program("*");
    assert_eq!(["*"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_equals() {
    let tokens = lex_program("==");
    assert_eq!(["=="], &tokens_to_literals(&tokens)[..]);
}

#[test]
#[should_panic]
fn invalid_token_should_raise_error() {
    let tokens = lex_program("=%");
    assert_eq!(0, tokens.len());
}

#[test]
fn several_tokens() {
    let tokens = lex_program(" + - ==  ;; *");
    assert_eq!(["+", "-", "==", ";", ";", "*"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn literal() {
    let tokens = lex_program("banana");
    assert_eq!(["banana"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn literal_and_token() {
    let tokens = lex_program("mango=");
    assert_eq!(["mango", "="], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn two_literals() {
    let tokens = lex_program("t omate");
    assert_eq!(["t", "omate"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn literal_number_and_string() {
    let tokens = lex_program(r#" banana 1337 "kiwi" "#);
    assert_eq!(["banana", "1337", "kiwi"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn complex_expression() {
    let tokens = lex_program("radio = pi*3 + 2;");
    assert_eq!(["radio", "=", "pi", "*", "3", "+", "2", ";"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn complex_expression_comparisons() {
    let tokens = lex_program("4 < 19>2 >= !1 <= 1!=10");
    assert_eq!(["4", "<", "19", ">", "2", ">=", "!", "1", "<=", "1", "!=", "10"],
               &tokens_to_literals(&tokens)[..]);
}

#[test]
fn parentheses_and_braces() {
    let tokens = lex_program("((2)) == !{{4}}");
    assert_eq!(["(", "(", "2", ")", ")", "==", "!", "{", "{", "4", "}", "}"],
               &tokens_to_literals(&tokens)[..]);
}

#[test]
fn single_letter_as_string() {
    let tokens = lex_program(r#""f""#);
    assert_eq!(["f"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn string_with_spaces() {
    let tokens = lex_program(r#""canada is a nice country""#);
    assert_eq!(["canada is a nice country"], &tokens_to_literals(&tokens)[..]);
}
#[test]
fn newlines() {
    let tokens = lex_program("3*\n2 + \n 3;");
    assert_eq!(["3", "*", "2", "+", "3", ";"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn keywords() {
    let tokens: Vec<Token> = lex_program("if print while return let fn");
    assert_eq!(["if", "print", "while", "return", "let", "fn"], &tokens_to_literals(&tokens)[..]);
    let token_types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        [TokenType::KeywordIf,
            TokenType::KeywordPrint,
            TokenType::KeywordWhile,
            TokenType::KeywordReturn,
            TokenType::KeywordLet,
            TokenType::KeywordFn
        ],
        &token_types[..]
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
                while y >= 12 {
                    print("answer is " y);
                }
                return 0;
            }
            "#,
    );

    assert_eq!(["fn","square","(","x",")","{","return","x","*","x",";","}",
               "fn","main","(",")","{","let","y","=","square","(","4",")",";",
               "while","y",">=","12","{","print","(","answer is ","y",")",";","}",
               "return","0",";","}"], &tokens_to_literals(&tokens)[..]);
}
