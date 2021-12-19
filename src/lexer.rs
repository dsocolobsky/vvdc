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
        Lexer {
            code: Vec::new(),
            tokens: Vec::new(),
            current: 0,
            current_char: '\0',
        }
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
            let c = self.next().to_string();

            let (tokentype, literal): (TokenType, String) = match c.as_str() {
                c if c.trim().is_empty() => {(TokenType::None, "".to_string())}
                "+" => (TokenType::Plus, c),
                "-" => (TokenType::Minus, c),
                "*" => (TokenType::Asterisk, c),
                ";" => (TokenType::Semicolon, c),
                "(" => (TokenType::Lparen, c),
                ")" => (TokenType::Rparen, c),
                "{" => (TokenType::Lbrace, c),
                "}" => (TokenType::Rbrace, c),
                "=" => {
                    if self.peek() == '=' {
                        self.next();
                        (TokenType::Equals, "==".to_string())
                    } else {
                        (TokenType::Assignment, c)
                    }
                }
                "!" => {
                    if self.peek() == '=' {
                        self.next();
                        (TokenType::Unequal, "!=".to_string())
                    } else {
                        (TokenType::Bang, c)
                    }
                }
                "<" => {
                    if self.peek() == '=' {
                        self.next();
                        (TokenType::Lteq, "<=".to_string())
                    } else {
                        (TokenType::Lt, c)
                    }
                }
                ">" => {
                    if self.peek() == '=' {
                        self.next();
                        (TokenType::Gteq, ">=".to_string())
                    } else {
                        (TokenType::Gt, c)
                    }
                }
                "\"" => self.scan_string(),
                c if c.chars().nth(0).unwrap().is_ascii_alphabetic() => self.scan_identifier(),
                c if c.chars().nth(0).unwrap().is_ascii_digit() => self.scan_number(),
                _ => panic!("unrecognized char: '{}'", c),
            };
            if tokentype != TokenType::None {
                self.tokens.push(Token::new(tokentype, literal));
            }
        }
    }

    fn scan_generic(&mut self, ttype: TokenType) -> String {
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
                break;
            }
            literal.push(c);
            self.next();
        }

        literal
    }

    fn scan_string(&mut self) -> (TokenType, String) {
        (TokenType::String, self.scan_generic(TokenType::String))
    }

    fn scan_identifier(&mut self) -> (TokenType, String) {
        let identifier = self.scan_generic(TokenType::Identifier);
        (Token::type_given_identifier(&identifier), identifier)
    }

    fn scan_number(&mut self) -> (TokenType, String) {
        (TokenType::Number, self.scan_generic(TokenType::Number))
    }
}

pub fn lex_program(program: &str) -> Vec<Token> {
    let mut lexer = Lexer::new();
    lexer.scan(program);

    return lexer.tokens;
}
