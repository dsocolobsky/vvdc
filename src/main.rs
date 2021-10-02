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
struct Token<'a> {
    token_type: TokenType,
    literal: &'a str,
}

impl Token<'_> {
    fn equals() -> Token<'static> {
        Token{token_type: TokenType::Equals, literal: "="}
    }

    fn plus() -> Token<'static> {
        Token{token_type: TokenType::Plus, literal: "+"}
    }

    fn minus() -> Token<'static> {
        Token{token_type: TokenType::Minus, literal: "-"}
    }

    fn asterisk() -> Token<'static> {
        Token{token_type: TokenType::Asterisk, literal: "*"}
    }

    fn semicolon() -> Token<'static> {
        Token{token_type: TokenType::Semicolon, literal: ";"}
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
            _ => {}
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
    fn parse_several_tokens() {
        let tokens = parse_program("=+-*+=;");
        assert_eq!([Token::equals(), Token::plus(), Token::minus(), Token::asterisk(), Token::plus(),
        Token::equals(), Token::semicolon()], &tokens[..]);
    }

}