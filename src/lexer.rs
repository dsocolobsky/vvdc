use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LexerState {
    Scanning,
    Literal,
    Number,
    String,
}

struct Lexer {
    code: Vec<char>,
    tokens: Vec<Token>,
    state: LexerState,
    current: usize,
    current_char: char,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {code: Vec::new(), tokens: Vec::new(), state: LexerState::Scanning, 
            current: 0, current_char: '\0'}
    }

    fn peek(&self) -> char {
        if self.current >= self.code.len() {
            '\0'
        } else {
            self.code[self.current]
        }
    }

    fn next(&mut self) -> char {
        let nc = self.peek();
        self.current += 1;
        self.current_char = nc;
        nc
    }

    fn scan(&mut self, program: &str) {
        self.code = program.chars().collect::<Vec<char>>();

        while self.current < self.code.len() {
            let c = self.next();
            println!("c = {}", c);

            match c {
                ' ' => {},
                '=' => self.tokens.push(Token::equals()),
                '+' => self.tokens.push(Token::plus()),
                '-' => self.tokens.push(Token::minus()),
                '*' => self.tokens.push(Token::asterisk()),
                ';' => self.tokens.push(Token::semicolon()),
                '"' => self.scan_string(),
                c if c.is_ascii_alphabetic() => self.scan_literal(),
                c if c.is_ascii_digit() => self.scan_number(),
                _ => panic!("unrecognized char: '{}'", c),
            }
        }
    }

    fn scan_generic(&mut self, ttype: TokenType) {
        let mut literal = String::from("");
        if ttype != TokenType::String {
            literal.push(self.current_char);
        }

        let mut c = self.current_char;
        while self.current <= self.code.len() {
            c = self.peek();
            if !c.is_ascii_alphanumeric() {
                break
            }
            literal.push(c);
            self.next();
        }

        let token = match ttype {
            TokenType::Literal => Token::literal(&literal),
            TokenType::String => Token::literal(&literal),
            TokenType::Number => Token::number(literal.parse::<i64>().unwrap()),
            _ => panic!("Should not be parsing {} as literal", literal),
        };

        self.tokens.push(token);
    }

    fn scan_string(&mut self) {
        self.scan_generic(TokenType::String);
    }

    fn scan_literal(&mut self) {
        self.scan_generic(TokenType::Literal);
    }

    fn scan_number(&mut self) {
        self.scan_generic(TokenType::Number);
    }
}

fn lex_program(program: &str) -> Vec<Token> {
    let mut lexer = Lexer::new();
    lexer.scan(program);

    return lexer.tokens;
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
