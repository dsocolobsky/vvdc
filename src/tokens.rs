
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
            LiteralType::Identifier(s) => write!(f, "l{:?}", s),
            LiteralType::Symbol(s) => write!(f, "{:?}", s),
            LiteralType::Number(n) => write!(f, "n{:?}", n),
            LiteralType::String(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(PartialEq)]
pub struct Token {
    token_type: TokenType,
    literal: Option<LiteralType>,
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

    pub fn assignment() -> Token {
        Token {
            token_type: TokenType::Assignment,
            literal: Some(LiteralType::Symbol("=".to_string())),
        }
    }

    pub fn plus() -> Token {
        Token {
            token_type: TokenType::Plus,
            literal: Some(LiteralType::Symbol("+".to_string())),
        }
    }

    pub fn minus() -> Token {
        Token {
            token_type: TokenType::Minus,
            literal: Some(LiteralType::Symbol("-".to_string())),
        }
    }

    pub fn asterisk() -> Token {
        Token {
            token_type: TokenType::Asterisk,
            literal: Some(LiteralType::Symbol("*".to_string())),
        }
    }

    pub fn semicolon() -> Token {
        Token {
            token_type: TokenType::Semicolon,
            literal: Some(LiteralType::Symbol(";".to_string())),
        }
    }

    pub fn equals() -> Token {
        Token {
            token_type: TokenType::Equals,
            literal: Some(LiteralType::Symbol("==".to_string())),
        }
    }

    pub fn bang() -> Token {
        Token {
            token_type: TokenType::Bang,
            literal: Some(LiteralType::Symbol("!".to_string())),
        }
    }

    pub fn unequal() -> Token {
        Token {
            token_type: TokenType::Unequal,
            literal: Some(LiteralType::Symbol("!=".to_string())),
        }
    }

    pub fn lt() -> Token {
        Token {
            token_type: TokenType::Lt,
            literal: Some(LiteralType::Symbol("<".to_string())),
        }
    }

    pub fn gt() -> Token {
        Token {
            token_type: TokenType::Gt,
            literal: Some(LiteralType::Symbol(">".to_string())),
        }
    }

    pub fn lteq() -> Token {
        Token {
            token_type: TokenType::Lteq,
            literal: Some(LiteralType::Symbol("<=".to_string())),
        }
    }

    pub fn gteq() -> Token {
        Token {
            token_type: TokenType::Gteq,
            literal: Some(LiteralType::Symbol(">=".to_string())),
        }
    }

    pub fn lparen() -> Token {
        Token {
            token_type: TokenType::Gteq,
            literal: Some(LiteralType::Symbol("(".to_string())),
        }
    }

    pub fn rparen() -> Token {
        Token {
            token_type: TokenType::Gteq,
            literal: Some(LiteralType::Symbol(")".to_string())),
        }
    }

    pub fn lbrace() -> Token {
        Token {
            token_type: TokenType::Lbrace,
            literal: Some(LiteralType::Symbol("{".to_string())),
        }
    }

    pub fn rbrace() -> Token {
        Token {
            token_type: TokenType::Rbrace,
            literal: Some(LiteralType::Symbol("}".to_string())),
        }
    }
}
