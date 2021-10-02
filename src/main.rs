use std::fmt;

#[derive(Debug, PartialEq)]
enum TokenType {
    Literal,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Semicolon
}

#[derive(PartialEq)]
struct Token {
    token_type: TokenType,
    literal: String,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.literal)
    }
}

impl Token {
    fn literal(literal: &str) -> Token {
        Token{token_type: TokenType::Literal, literal: literal.to_string()}
    }

    fn equals() -> Token {
        Token{token_type: TokenType::Equals, literal: "=".to_string()}
    }

    fn plus() -> Token {
        Token{token_type: TokenType::Plus, literal: "+".to_string()}
    }

    fn minus() -> Token {
        Token{token_type: TokenType::Minus, literal: "-".to_string()}
    }

    fn asterisk() -> Token {
        Token{token_type: TokenType::Asterisk, literal: "*".to_string()}
    }

    fn semicolon() -> Token {
        Token{token_type: TokenType::Semicolon, literal: ";".to_string()}
    }
}

fn parse_program(program: &str) -> Vec<Token> {
    println!("Parsing: {:?}", program);
    let mut parsing_literal = false;
    let mut last_char: Option<char> = None;
    let mut tokens:Vec<Token> = Vec::new();
    let mut iter = program.chars().peekable();

    while let Some(c) = iter.next() {

        if parsing_literal {
            let last_ch = last_char.expect("last char should not be empty");
            let mut literal = String::from("");
            // We know last and current must be literal chars
            literal.push(last_ch);
            literal.push(c);

            match iter.peek() {
                Some(nc) => {
                    // Case when it's a 2-char length literal
                    if !nc.is_ascii_alphanumeric() {
                        println!("parsed literal: {}", literal);
                        tokens.push(Token::literal(&literal));
                        parsing_literal = false;
                        continue;
                    }
                },
                None => break, // End of the program, exit.
            }
    
            // Here we know for sure the next char is still part of the literal
            while let Some(c) = iter.next() {
                literal.push(c);
        
                if let Some(nc) = iter.peek() {
                    if !nc.is_ascii_alphanumeric() {
                        //parsing_literal = false;
                        break;
                    }
                }
            }
            
            println!("parsed literal: {}", literal);
            tokens.push(Token::literal(&literal));
            parsing_literal = false;
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
            c if c.is_ascii_alphanumeric() => {
                println!("starting parse literal");
                if let Some(nc) = iter.peek() {
                    if !(*nc).is_ascii_alphanumeric() {
                        tokens.push(Token::literal(&String::from(c)));
                        continue;
                    } else {
                        parsing_literal = true;
                    }
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

}