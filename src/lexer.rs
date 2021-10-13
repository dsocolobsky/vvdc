use crate::tokens::Token;
use crate::tokens::TokenType;

struct Lexer {
    code: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    current_char: char,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {code: Vec::new(), tokens: Vec::new(), 
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

            match c {
                c if c.is_whitespace() => {},
                '+' => self.tokens.push(Token::plus()),
                '-' => self.tokens.push(Token::minus()),
                '*' => self.tokens.push(Token::asterisk()),
                ';' => self.tokens.push(Token::semicolon()),
                '(' => self.tokens.push(Token::lparen()),
                ')' => self.tokens.push(Token::rparen()),
                '{' => self.tokens.push(Token::lbrace()),
                '}' => self.tokens.push(Token::rbrace()),
                '=' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::equals());
                        self.next();
                    }  else {
                        self.tokens.push(Token::assignment());
                    }
                },
                '!' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::unequal());
                        self.next();
                    } else {
                        self.tokens.push(Token::bang());
                    }
                },
                '<' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::lteq());
                        self.next();
                    } else {
                        self.tokens.push(Token::lt());
                    }
                },
                '>' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::gteq());
                        self.next();
                    } else {
                        self.tokens.push(Token::gt());
                    }
                },
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

        while self.current <= self.code.len() {
            let c = self.peek();
            if !c.is_ascii_alphanumeric() && !(ttype == TokenType::String && c.is_whitespace()) {
                if c == '"' && ttype == TokenType::String {
                    self.next();
                }
                break
            }
            literal.push(c);
            self.next();
        }

        let token = match ttype {
            TokenType::Literal => {
                let maybe_token: Option<Token> = Token::keyword(&literal);
                if let Some(tk) = maybe_token {
                    tk
                } else {
                    Token::literal(&literal)
                }
            },
            TokenType::String => Token::string(&literal),
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

pub fn lex_program(program: &str) -> Vec<Token> {
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
    }}

    #[test]
    fn comparison_operators() {
        let tokens = lex_program("4 < 19>2 >= !1 <= 1!=10");
        assert_eq!([
            Token::number(4), Token::lt(), Token::number(19), Token::gt(), 
            Token::number(2), Token::gteq(), Token::bang(), Token::number(1), Token::lteq(),
            Token::number(1), Token::unequal() ,Token::number(10)
            ], &tokens[..]);
    }

    #[test]
    fn parentheses() {
        let tokens = lex_program("(2) == !((4))");
        assert_eq!([Token::lparen(), Token::number(2), Token::rparen(), Token::equals(),
            Token::bang(), Token::lparen(), Token::lparen(), Token::number(4), Token::rparen(),
            Token::rparen()], &tokens[..]);
    }

    #[test]
    fn braces() {
        let tokens = lex_program("fn{{}}");
        assert_eq!([Token::literal("fn"), Token::lbrace(), Token::lbrace(),
        Token::rbrace(), Token::rbrace()], &tokens[..]);
    }

    #[test]
    fn newlines() {
        let tokens = lex_program("3*\n2 + \n 3;");
        assert_eq!([Token::number(3), Token::asterisk(), Token::number(2), Token::plus(),
            Token::number(3), Token::semicolon()], &tokens[..]);
    }

    #[test]
    fn keywords() {
        let tokens: Vec<Token> = lex_program("if print while return");
        let tokens: Vec<TokenType> = tokens.into_iter().map(|token| {token.token_type}).collect();
        assert_eq!([TokenType::KeywordIf, TokenType::KeywordPrint, TokenType::KeywordWhile, TokenType::KeywordReturn],
            &tokens[..]);
    }

    #[test]
    fn sample_program() {
        let tokens = lex_program(
            r#"
            square(x) {
                return x * x;
            }

            main() {
                y = square(4);
                while y > 12 {
                    print("answer is " y);
                }
                return 0;
            }
            "#);

        assert_eq!([Token::literal("square"), Token::lparen(), Token::literal("x"), Token::rparen(),
                    Token::lbrace(), Token::keyword_return(), Token::literal("x"), Token::asterisk(),
                    Token::literal("x"), Token::semicolon(), Token::rbrace(), Token::literal("main"), Token::lparen(),
                    Token::rparen(), Token::lbrace(), Token::literal("y"), Token::assignment(),
                    Token::literal("square"), Token::lparen(), Token::number(4), Token::rparen(),
                    Token::semicolon(), Token::keyword_while(), Token::literal("y"), Token::gt(),
                    Token::number(12), Token::lbrace(), Token::keyword_print(), Token::lparen(),
                    Token::string("answer is "), Token::literal("y"), Token::rparen(), Token::semicolon(), Token::rbrace(),
                    Token::keyword_return(), Token::number(0), Token::semicolon(), Token::rbrace()], &tokens[..]);
    }
