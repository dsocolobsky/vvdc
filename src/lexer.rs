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
            let c = self.next();

            match c {
                c if c.is_whitespace() => {}
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
                    } else {
                        self.tokens.push(Token::assignment());
                    }
                }
                '!' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::unequal());
                        self.next();
                    } else {
                        self.tokens.push(Token::bang());
                    }
                }
                '<' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::lteq());
                        self.next();
                    } else {
                        self.tokens.push(Token::lt());
                    }
                }
                '>' => {
                    if self.peek() == '=' {
                        self.tokens.push(Token::gteq());
                        self.next();
                    } else {
                        self.tokens.push(Token::gt());
                    }
                }
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
                break;
            }
            literal.push(c);
            self.next();
        }

        let token = match ttype {
            TokenType::Literal => Token::keyword_or_literal(&literal),
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
