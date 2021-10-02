#[derive(Debug, PartialEq)]
enum TokenType {
    Literal,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Semicolon
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    literal: String,
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
    let mut tokens:Vec<Token> = Vec::new();
    for c in program.chars() {
        match c {
            '=' => tokens.push(Token::equals()),
            '+' => tokens.push(Token::plus()),
            '-' => tokens.push(Token::minus()),
            '*' => tokens.push(Token::asterisk()),
            ';' => tokens.push(Token::semicolon()),
            ' ' => {},
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

}