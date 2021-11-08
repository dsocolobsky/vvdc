use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    LiteralExpression,
    PrefixExpression
}

pub struct Expression  {
    pub expression_type: ExpressionType,
    pub token: Token,
    pub right: Option<Box<Expression>>,
}

impl Expression {
    fn literal_expression(token: Token) -> Expression {
        Expression {
            expression_type: ExpressionType::LiteralExpression,
            token: token,
            right: None, 
        }
    }

    fn  prefix_expression(token: Token, right: Expression) -> Expression {
        Expression {
            expression_type: ExpressionType::PrefixExpression,
            token: token,
            right: Some(Box::new(right)), 
        }
    }
}

pub fn parse(tokens: &Vec<Token>) ->Vec<Expression> {
    let mut expressions = Vec::new();
    let mut index: usize = 0;

    while index < tokens.len() {
        let token = &tokens[index];
        match token.token_type {
            TokenType::Bang => {
                expressions.push(Expression::prefix_expression(
                    tokens[index].clone(),
                    Expression::literal_expression(tokens[index+1].clone()
                )));
                index = index + 2;
            },
            TokenType::Literal => todo!(),
            TokenType::String => todo!(),
            TokenType::Number => todo!(),
            TokenType::Assignment => todo!(),
            TokenType::Plus => todo!(),
            TokenType::Minus => todo!(),
            TokenType::Asterisk => todo!(),
            TokenType::Semicolon => todo!(),
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
            TokenType::KeywordReturn => todo!(),
            TokenType::KeywordWhile => todo!(),
            TokenType::KeywordLet => todo!(),
            TokenType::KeywordFn => todo!(),
        };
    }
    
    expressions
}