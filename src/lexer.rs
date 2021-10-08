use crate::tokens::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LexerState {
    Scanning,
    Literal,
    Number,
    PreString,
    String,
    PostString,
}

fn lex_single_character(c: char) -> Token {
    if c.is_ascii_alphabetic() {
        println!("Parsing single character");
        Token::literal(&String::from(c))
    } else if c.is_ascii_digit() {
        println!("Parsing single digit");
        let digit = (String::from(c)).parse::<i64>().unwrap();
        Token::number(digit)
    } else {
        panic!("{:?} is neither numeric nor alphabetic!", c);
    }
}

fn token_given_lexer_state(buf: &str, state: LexerState) -> Token {
    match state {
        LexerState::Literal => {
            println!("parsed literal: {}", buf);
            Token::literal(&buf)
        }
        LexerState::Number => {
            let as_number = buf.parse::<i64>().unwrap();
            println!("parsed number: {}", as_number);
            Token::number(as_number)
        }
        LexerState::String => {
            println!("parsed string: {}", buf);
            Token::string(&buf)
        }
        _ => {
            panic!("panic parsing {:?}", buf)
        }
    }
}

fn lex_program(program: &str) -> Vec<Token> {
    println!("Parsing: {:?}", program);
    let mut parser_state = LexerState::Scanning;
    let mut last_char: Option<char> = None;
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = program.chars().peekable();

    while let Some(c) = iter.next() {
        println!("ch: {:?}", c);

        if matches!(
            parser_state,
            LexerState::Literal | LexerState::Number | LexerState::String
        ) {
            let last_ch = last_char.expect("last char should not be empty");
            let mut literal = String::from("");
            // We know last and current must be literal chars
            if parser_state != LexerState::String {
                literal.push(last_ch);
                println!("push (lch): {:?}", last_ch);
            }
            literal.push(c);
            println!("push: {:?}", c);

            match iter.peek() {
                Some(nc) => {
                    // Case when it's a 2-char length literal
                    if !nc.is_ascii_alphanumeric() {
                        tokens.push(token_given_lexer_state(&literal, parser_state));
                        parser_state = if parser_state == LexerState::String {
                            LexerState::PostString
                        } else {
                            LexerState::Scanning
                        };
                        continue;
                    }
                }
                None => {} // End of the program, skip and finish.
            }

            // Here we know for sure the next char is still part of the literal
            while let Some(c) = iter.next() {
                if c != '"' {
                    literal.push(c);
                }

                if let Some(nc) = iter.peek() {
                    if !nc.is_ascii_alphanumeric()
                        && (parser_state != LexerState::String && *nc != '"')
                    {
                        //parsing_literal = false;
                        break;
                    }
                }
            }

            tokens.push(token_given_lexer_state(&literal, parser_state));
            parser_state = if parser_state == LexerState::String {
                println!("fake parsing string");
                LexerState::PostString
            } else {
                LexerState::Scanning
            };
            continue;
        }

        last_char = Some(c);
        match c {
            '=' => {
                println!("Equals");
                tokens.push(Token::equals());
            }
            '+' => {
                println!("Plus");
                tokens.push(Token::plus());
            }
            '-' => {
                println!("Minus");
                tokens.push(Token::minus());
            }
            '*' => {
                println!("Asterisk");
                tokens.push(Token::asterisk());
            }
            ';' => {
                println!("Semicolon");
                tokens.push(Token::semicolon());
            }
            ' ' => {
                println!("space")
            }
            '"' => {
                if matches!(
                    parser_state,
                    LexerState::String | LexerState::PostString
                ) {
                    println!("end of string");
                    parser_state = LexerState::Scanning;
                } else {
                    println!("start of string");
                    parser_state = LexerState::String;
                }
            }
            c if c.is_ascii_alphanumeric() => {
                if let Some(nc) = iter.peek() {
                    if !(*nc).is_ascii_alphanumeric() {
                        tokens.push(lex_single_character(c));
                    } else {
                        parser_state = if parser_state == LexerState::PreString {
                            println!("keep parsing string");
                            LexerState::String
                        } else if (*nc).is_ascii_alphabetic() {
                            println!("Start parse literal");
                            LexerState::Literal
                        } else if (*nc).is_ascii_digit() {
                            println!("Start parse number");
                            LexerState::Number
                        } else {
                            panic!("{:?} is neither alphabetic nor number", nc)
                        }
                    }
                } else {
                    tokens.push(lex_single_character(c));
                }
            }
            _ => panic!("unrecognized: '{}'", c),
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_produces_empty_list_of_tokens() {
        let tokens = lex_program("");
        assert_eq!(0, tokens.len());
    }

    #[test]
    fn token_equals() {
        let tokens = lex_program("=");
        assert_eq!([Token::equals()], &tokens[..]);
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
                Token::equals(),
                Token::plus(),
                Token::minus(),
                Token::asterisk(),
                Token::plus(),
                Token::equals(),
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
        assert_eq!([Token::literal("mango"), Token::equals()], &tokens[..]);
    }

    #[test]
    fn two_literals() {
        let tokens = lex_program("t omate");
        assert_eq!([Token::literal("t"), Token::literal("omate")], &tokens[..]);
    }

    #[test]
    fn complex_expression() {
        let tokens = lex_program("radio = pi*e;");
        assert_eq!(
            [
                Token::literal("radio"),
                Token::equals(),
                Token::literal("pi"),
                Token::asterisk(),
                Token::literal("e"),
                Token::semicolon()
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
        assert_eq!([Token::equals(), Token::number(17)], &tokens[..]);
    }

    #[test]
    fn complex_expression_with_numbers() {
        let tokens = lex_program("x = 4 * 35+2 1;");
        assert_eq!(
            [
                Token::literal("x"),
                Token::equals(),
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
    }}
