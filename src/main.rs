use std::fmt;

#[derive(Debug, PartialEq)]
enum TokenType {
    Literal,
    String,
    Number,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Semicolon
}

#[derive(PartialEq)]
enum LiteralType {
    Identifier(String),
    Symbol(String),
    Number(i64),
    String(String),
}

impl fmt::Debug for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LiteralType::Identifier(s) => write!(f, "i{:?}", s),
            LiteralType::Symbol(s) => write!(f, "{:?}", s),
            LiteralType::Number(n) => write!(f, "n{:?}", n),
            LiteralType::String(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(PartialEq)]
struct Token {
    token_type: TokenType,
    literal: Option<LiteralType>,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.literal.as_ref().unwrap())
    }
}

impl Token {
    fn literal(literal: &str) -> Token {
        Token {
            token_type: TokenType::Literal,
            literal: Some(LiteralType::Identifier(literal.to_string()))
        }
    }

    fn number(n: i64) -> Token {
        Token {
            token_type: TokenType::Number, 
            literal: Some(LiteralType::Number(n)),
        }
    }

    fn string(string: &str) -> Token {
        Token {
            token_type: TokenType::String,
            literal: Some(LiteralType::String(string.to_string()))
        }
    }

    fn equals() -> Token {
        Token {
            token_type: TokenType::Equals,
             literal: Some(LiteralType::Symbol("=".to_string()))
        }
    }

    fn plus() -> Token {
        Token {
            token_type: TokenType::Plus,
             literal: Some(LiteralType::Symbol("+".to_string()))
        }
    }

    fn minus() -> Token {
        Token {
            token_type: TokenType::Minus,
             literal: Some(LiteralType::Symbol("-".to_string()))
        }
    }

    fn asterisk() -> Token {
        Token {
            token_type: TokenType::Asterisk,
             literal: Some(LiteralType::Symbol("*".to_string()))
        }
    }

    fn semicolon() -> Token {
        Token {
            token_type: TokenType::Semicolon,
            literal: Some(LiteralType::Symbol(";".to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParserState {
    Scanning,
    ParsingLiteral,
    ParsingNumber,
    PreParsingString,
    ParsingString,
    PostParsingString,
}

fn parse_single_character(c: char) -> Token {
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

fn token_given_parser_state(buf: &str, state: ParserState) -> Token {
    match state {
        ParserState::ParsingLiteral => {
            println!("parsed literal: {}", buf);
            Token::literal(&buf)
        },
        ParserState::ParsingNumber => {
            let as_number = buf.parse::<i64>().unwrap();
            println!("parsed number: {}", as_number);
            Token::number(as_number)
        },
        ParserState::ParsingString => {
            println!("parsed string: {}", buf);
            Token::string(&buf)
        }
        _ => {panic!("panic parsing {:?}", buf)},
    }
}

fn parse_program(program: &str) -> Vec<Token> {
    println!("Parsing: {:?}", program);
    let mut parser_state = ParserState::Scanning;
    let mut last_char: Option<char> = None;
    let mut tokens:Vec<Token> = Vec::new();
    let mut iter = program.chars().peekable();

    while let Some(c) = iter.next() {
        println!("ch: {:?}", c);

        if matches!(parser_state, 
            ParserState::ParsingLiteral | ParserState::ParsingNumber | ParserState::ParsingString) {
            let last_ch = last_char.expect("last char should not be empty");
            let mut literal = String::from("");
            // We know last and current must be literal chars
            if parser_state != ParserState::ParsingString {
                literal.push(last_ch);
                println!("push (lch): {:?}", last_ch);
            }
            literal.push(c);
            println!("push: {:?}", c);

            match iter.peek() {
                Some(nc) => {
                    // Case when it's a 2-char length literal
                    if !nc.is_ascii_alphanumeric() {
                        tokens.push(token_given_parser_state(&literal, parser_state));
                        parser_state = if parser_state == ParserState::ParsingString {
                            ParserState::PostParsingString
                        } else {
                            ParserState::Scanning
                        };
                        continue;
                    }
                },
                None => {}, // End of the program, skip and finish.
            }
    
            // Here we know for sure the next char is still part of the literal
            while let Some(c) = iter.next() {
                if c != '"' {
                    literal.push(c);
                }
        
                if let Some(nc) = iter.peek() {
                    if !nc.is_ascii_alphanumeric() && (parser_state != ParserState::ParsingString && *nc !='"') {
                        //parsing_literal = false;
                        break;
                    }
                }
            }

            tokens.push(token_given_parser_state(&literal, parser_state));
            parser_state = if parser_state == ParserState::ParsingString {
                println!("fake parsing string");
                ParserState::PostParsingString
            } else {
                ParserState::Scanning
            };
            continue;
        }

        last_char = Some(c);
        match c {
            '=' => { println!("Equals"); tokens.push(Token::equals()); },
            '+' => { println!("Plus"); tokens.push(Token::plus()); },
            '-' => { println!("Minus"); tokens.push(Token::minus()); },
            '*' => { println!("Asterisk"); tokens.push(Token::asterisk()); },
            ';' => { println!("Semicolon"); tokens.push(Token::semicolon()); },
            ' ' => {println!("space")},
            '"' => {
                if matches!(parser_state, ParserState::ParsingString | ParserState::PostParsingString) {
                    println!("end of string");
                    parser_state = ParserState::Scanning;
                } else {
                    println!("start of string");
                    parser_state = ParserState::ParsingString;
                }
                
            },
            c if c.is_ascii_alphanumeric() => {
                if let Some(nc) = iter.peek() {
                    if !(*nc).is_ascii_alphanumeric() {
                        tokens.push(parse_single_character(c));
                    } else {
                        parser_state = if parser_state == ParserState::PreParsingString {
                            println!("keep parsing string");
                            ParserState::ParsingString
                        } else if (*nc).is_ascii_alphabetic() {
                            println!("Start parse literal");
                            ParserState::ParsingLiteral
                        } else if (*nc).is_ascii_digit() {
                            println!("Start parse number");
                            ParserState::ParsingNumber
                        } else {
                            panic!("{:?} is neither alphabetic nor number", nc)
                        }
                    }
                } else {
                    tokens.push(parse_single_character(c));
                }
            },
            _ => panic!("unrecognized: '{}'", c),
        }
    }
    return tokens;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string_produces_empty_list_of_tokens() {
        let tokens = parse_program("");
        assert_eq!(0, tokens.len());
    }

    #[test]
    fn parse_semicolon() {
        let tokens = parse_program("=");
        assert_eq!([Token::equals()], &tokens[..]);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_token_should_raise_error() {
        let tokens = parse_program("=%");
        assert_eq!(0, tokens.len());
    }

    #[test]
    fn parse_several_tokens() {
        let tokens = parse_program("=+-*+=;");
        assert_eq!([Token::equals(), Token::plus(), Token::minus(), Token::asterisk(), Token::plus(),
        Token::equals(), Token::semicolon()], &tokens[..]);
    }

    #[test]
    fn parse_several_tokens_with_spaces() {
        let tokens = parse_program(" + -    ;; *");
        assert_eq!([Token::plus(), Token::minus(), Token::semicolon(), 
            Token::semicolon(), Token::asterisk()], &tokens[..]);
    }

    #[test]
    fn parse_literal() {
        let tokens = parse_program("banana");
        assert_eq!([Token::literal("banana")], &tokens[..]);
    }

    #[test]
    fn parse_single_letter_literal() {
        let tokens = parse_program("a");
        assert_eq!([Token::literal("a")], &tokens[..]);
    }

    #[test]
    fn parse_literal_and_token() {
        let tokens = parse_program("mango=");
        assert_eq!([Token::literal("mango"), Token::equals()], &tokens[..]);
    }

    #[test]
    fn parse_two_literals() {
        let tokens = parse_program("t omate");
        assert_eq!([Token::literal("t"), Token::literal("omate")], &tokens[..]);
    }

    #[test]
    fn parse_complex_expression() {
        let tokens = parse_program("radio = pi*e;");
        assert_eq!([Token::literal("radio"), Token::equals(), Token::literal("pi"),
        Token::asterisk(), Token::literal("e"), Token::semicolon()], &tokens[..]);
    }

    #[test]
    fn parse_a_single_digit_number() {
        let tokens = parse_program("4");
        assert_eq!([Token::number(4)], &tokens[..]);
    }

    #[test]
    fn parse_a_multiple_digit_number() {
        let tokens = parse_program("1337");
        assert_eq!([Token::number(1337)], &tokens[..]);
    }

    #[test]
    fn parse_number_and_symbol() {
        let tokens = parse_program("4*32");
        assert_eq!([Token::number(4), Token::asterisk(), Token::number(32)], &tokens[..]);
    }

    #[test]
    fn parse_symbol_and_number() {
        let tokens = parse_program("=17");
        assert_eq!([Token::equals(), Token::number(17)], &tokens[..]);
    }

    #[test]
    fn parse_complex_expression_with_numbers() {
        let tokens = parse_program("x = 4 * 35+2 1;");
        assert_eq!([Token::literal("x"), Token::equals(), Token::number(4), Token::asterisk(),
                    Token::number(35), Token::plus(), Token::number(2), Token::number(1), Token::semicolon()], 
                    &tokens[..]);
    }

    #[test]
    fn parse_single_letter_as_string() {
        let tokens = parse_program(r#""f""#);
        assert_eq!([Token::string("f")], &tokens[..]);
    }

    #[test]
    fn parse_single_string() {
        let tokens = parse_program(r#""cafe""#);
        assert_eq!([Token::string("cafe")], &tokens[..]);
    }

    #[test]
    fn parse_string_with_spaces() {
        let tokens = parse_program(r#""canada nice country""#);
        assert_eq!([Token::string("canada nice country")], &tokens[..]);
    }
    
}