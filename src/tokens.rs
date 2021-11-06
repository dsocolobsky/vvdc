
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Literal,
    String,
    Number,
    Assignment,
    Plus,
    Minus,
    Asterisk,
    Semicolon,
    Bang,
    Equals,
    Unequal,
    Lt,
    Gt,
    Lteq,
    Gteq,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    KeywordIf,
    KeywordPrint,
    KeywordReturn,
    KeywordWhile,
    KeywordLet,
}

#[derive(PartialEq)]
pub enum LiteralType {
    Identifier(String),
    Symbol(String),
    Number(i64),
    String(String),
}

impl fmt::Debug for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LiteralType::Identifier(s) => write!(f, "{}", s),
            LiteralType::Symbol(s) => write!(f, "{}", s),
            LiteralType::Number(n) => write!(f, "{}", n),
            LiteralType::String(s) => write!(f, r#""{}""#, s),
        }
    }
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<LiteralType>,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.literal.as_ref().unwrap())
    }
}

impl Token {
    pub fn literal(literal: &str) -> Token {
        Token {
            token_type: TokenType::Literal,
            literal: Some(LiteralType::Identifier(literal.to_string())),
        }
    }

    pub fn number(n: i64) -> Token {
        Token {
            token_type: TokenType::Number,
            literal: Some(LiteralType::Number(n)),
        }
    }

    pub fn string(string: &str) -> Token {
        Token {
            token_type: TokenType::String,
            literal: Some(LiteralType::String(string.to_string())),
        }
    }

    fn create_symbol(ttype: TokenType, literal: &str) -> Token {
        Token {
            token_type: ttype,
            literal: Some(LiteralType::Symbol(literal.to_string())),
        }
    }

    pub fn plus() -> Token {
        Token::create_symbol(TokenType::Plus, "+")
    }

    pub fn minus() -> Token {
        Token::create_symbol(TokenType::Minus, "-")
    }

    pub fn asterisk() -> Token {
        Token::create_symbol(TokenType::Asterisk, "*")
    }

    pub fn semicolon() -> Token {
        Token::create_symbol(TokenType::Semicolon, ";")
    }

    pub fn assignment() -> Token {
        Token::create_symbol(TokenType::Assignment, "=")
    }

    pub fn equals() -> Token {
        Token::create_symbol(TokenType::Equals, "==")
    }

    pub fn bang() -> Token {
        Token::create_symbol(TokenType::Bang, "!")
    }

    pub fn unequal() -> Token {
        Token::create_symbol(TokenType::Unequal, "!=")
    }

    pub fn lt() -> Token {
        Token::create_symbol(TokenType::Lt, "<")
    }

    pub fn gt() -> Token {
        Token::create_symbol(TokenType::Gt, ">")
    }

    pub fn lteq() -> Token {
        Token::create_symbol(TokenType::Lteq, "<=")
    }

    pub fn gteq() -> Token {
        Token::create_symbol(TokenType::Gteq, ">=")
    }

    pub fn lparen() -> Token {
        Token::create_symbol(TokenType::Lparen, "(")
    }

    pub fn rparen() -> Token {
        Token::create_symbol(TokenType::Rparen, ")")
    }

    pub fn lbrace() -> Token {
        Token::create_symbol(TokenType::Lbrace, "{")
    }

    pub fn rbrace() -> Token {
        Token::create_symbol(TokenType::Rbrace, "}")
    }

    pub fn keyword(key: &str) -> Option<Token> {
        let ttype = match &*key {
            "if" => Some(TokenType::KeywordIf),
            "print" => Some(TokenType::KeywordPrint),
            "return" => Some(TokenType::KeywordReturn),
            "while" => Some(TokenType::KeywordWhile),
            "let" => Some(TokenType::KeywordLet),
            &_ => None,
        };
        if let Some(t) = ttype {
            Some(Token{token_type: t, literal: Some(LiteralType::String(key.to_string()))})
        } else {
            None
        }
    }

    pub fn keyword_if() -> Token {
        Token::keyword("if").unwrap()
    }

    pub fn keyword_print() -> Token {
        Token::keyword("print").unwrap()
    }

    pub fn keyword_return() -> Token {
        Token::keyword("return").unwrap()
    }

    pub fn keyword_while() -> Token {
        Token::keyword("while").unwrap()
    }

    pub fn keyword_let() -> Token {
        Token::keyword("let").unwrap()
    }
    
}
