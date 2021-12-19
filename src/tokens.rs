use std::fmt;
use crate::tokens::TokenType::{Identifier, KeywordFn, KeywordIf, KeywordLet, KeywordPrint, KeywordReturn, KeywordWhile};

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Identifier, String, Number,
    Assignment, Plus, Minus, Asterisk,
    Semicolon,
    Bang, Equals, Unequal, Lt, Gt, Lteq, Gteq,
    Lparen, Rparen, Lbrace, Rbrace,
    KeywordIf, KeywordPrint, KeywordReturn, KeywordWhile, KeywordLet, KeywordFn,
    None,
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.literal)
    }
}

impl Token {

    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal: literal,
        }
    }

    pub fn new_with_identifier(literal: &str) -> Token {
        let token_type = Self::type_given_identifier(literal);
        Token {
            token_type,
            literal: literal.to_string()
        }
    }

    pub fn type_given_identifier(identifier: &str) -> TokenType {
        match identifier {
            "if" => KeywordIf,
            "print" => KeywordPrint,
            "return" => KeywordReturn,
            "while" => KeywordWhile,
            "let" => KeywordLet,
            "fn" => KeywordFn,
            _ => Identifier,
        }
    }

    pub fn to_numeric(&self) -> i64 {
        match self.token_type {
            TokenType::Number => self.literal.parse::<i64>().unwrap(),
            _ => panic!("attempting to call to_numeric on a non number")
        }
    }
}
