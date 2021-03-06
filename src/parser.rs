use std::fmt::Debug;
use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Identifier,
    Number,
    String,
    Prefix,
    Infix,
    Return,
}

pub trait Expression {
    fn get_type(&self) -> ExpressionType;
    fn as_str(&self) -> String;

    fn literal(&self) -> Option<String> {
        None
    }

    fn left(&self) -> Option<&BoxExpression> {
        None
    }

    fn right(&self) -> Option<&BoxExpression> {
        None
    }
}

pub type BoxExpression = Box<dyn Expression>;

#[derive(Debug, Clone)]
pub struct IdentifierExpression {
    pub token: Token,
    pub value: i64,
}

impl Expression for IdentifierExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Identifier
    }

    fn as_str(&self) -> String {
        self.value.to_string()
    }

    fn literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }
}

#[derive(Debug, Clone)]
pub struct NumberLiteralExpression {
    pub token: Token,
    pub value: i64,
}

impl Expression for NumberLiteralExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Number
    }

    fn as_str(&self) -> String {
        self.value.to_string()
    }

    fn literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteralExpression {
    pub token: Token,
    pub value: String,
}

impl Expression for StringLiteralExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::String
    }

    fn as_str(&self) -> String {
        self.value.to_string()
    }

    fn literal(&self) -> Option<String> {
        Some(self.token.literal.clone())
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub right: BoxExpression,
}

impl Debug for PrefixExpression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PrefixExpression")
    }
}

impl Expression for PrefixExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Prefix
    }

    fn as_str(&self) -> String {
        format!("{:?}", self.token.token_type)
    }

    fn right(&self) -> Option<&BoxExpression> {
        Some(&self.right)
    }
}

pub struct InfixExpression {
    pub token: Token,
    pub left: BoxExpression,
    pub right: BoxExpression,
}

impl Debug for InfixExpression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "InfixExpression")
    }
}

impl Expression for InfixExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Infix
    }

    fn as_str(&self) -> String {
        format!("{:?}", self.token.token_type)
    }

    fn left(&self) -> Option<&BoxExpression> {
        Some(&self.left)
    }

    fn right(&self) -> Option<&BoxExpression> {
        Some(&self.right)
    }
}

pub struct ReturnExpression {
    pub token: Token,
    pub value: BoxExpression,
}

impl Debug for ReturnExpression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Return")
    }
}

impl Expression for ReturnExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Return
    }

    fn as_str(&self) -> String {
        format!("{:?}", self.token.token_type)
    }

    fn right(&self) -> Option<&BoxExpression> {
        Some(&self.value)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    expressions: Vec<BoxExpression>,
    token_index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            expressions: Vec::new(),
            token_index: 0,
        }
    }

    pub fn parse(&mut self) {
        while self.token_index < self.tokens.len() {
            let (expression, advance) = self.parse_expression(self.token_index);
            if let Some(exp) = expression {
                self.expressions.push(exp);
            }
            self.token_index = self.token_index + advance;
        }
    }

    fn peek_next(&self, from: usize) -> Option<Token> {
        if (from + 1) < self.tokens.len() {
            Some(self.tokens[from + 1].clone())
        } else {
            None
        }
    }

    fn parse_infix_expression(&self, left: BoxExpression, from: usize) -> (BoxExpression, usize) {
        let (right, adv) = self.parse_expression(from + 1);
        let infix = InfixExpression {
            token: self.tokens[from].clone(),
            left,
            right: right.unwrap()
        };
        (Box::new(infix), adv + 1)
    }

    fn parse_expression(&self, from: usize) -> (Option<BoxExpression>, usize) {
        let token = &self.tokens[from];
        match token.token_type {
            TokenType::Bang => {
                let (rhs, adv) = self.parse_expression(from + 1);
                let expression = PrefixExpression {
                    token: self.tokens[from].clone(),
                    right: rhs.unwrap(),
                };
                return (Some(Box::new(expression)), 1 + adv);
            }
            TokenType::String => {
                let expression = StringLiteralExpression {
                    token: self.tokens[from].clone(),
                    value: self.tokens[from].literal.clone(),
                };
                return (Some(Box::new(expression)), 1);
            },
            TokenType::Number => {
                let expression = NumberLiteralExpression {
                    token: self.tokens[from].clone(),
                    value: self.tokens[from].to_numeric(),
                };
                return (Some(Box::new(expression)), 1);
            }
            TokenType::Identifier => todo!(),
            TokenType::Assignment => todo!(),
            TokenType::Plus | TokenType::Minus => {
                let (lhs, _) = self.parse_expression(from - 1);
                let (rhs, adv) = self.parse_expression(from + 1);

                let expression = InfixExpression {
                    token: self.tokens[from].clone(),
                    left: lhs.unwrap(),
                    right: rhs.unwrap(),
                };
                return (Some(Box::new(expression)), 1 + adv);
            }
            TokenType::Asterisk => todo!(),
            TokenType::Semicolon => return (None, 1),
            TokenType::Equals => todo!(),
            TokenType::Unequal => todo!(),
            TokenType::Lt => todo!(),
            TokenType::Gt => todo!(),
            TokenType::Lteq => todo!(),
            TokenType::Gteq => todo!(),
            TokenType::Lparen => todo!(),
            TokenType::Rparen => todo!(),
            TokenType::Lbrace => todo!(),
            TokenType::Rbrace => todo!(),
            TokenType::KeywordIf => todo!(),
            TokenType::KeywordPrint => todo!(),
            TokenType::KeywordReturn => {
                let (right_expression, adv) = self.parse_expression(self.token_index + 1);
                let expression = ReturnExpression {
                    token: self.tokens[from].clone(),
                    value: right_expression.unwrap(),
                };
                return (Some(Box::new(expression)), 1 + adv);
            }
            TokenType::KeywordWhile => todo!(),
            TokenType::KeywordLet => todo!(),
            TokenType::KeywordFn => todo!(),
            TokenType::None => ((None), 0),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<BoxExpression> {
    let mut parser = Parser::new(tokens);
    parser.parse();
    parser.expressions
}
