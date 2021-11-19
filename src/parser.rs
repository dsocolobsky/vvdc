use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    LiteralExpression,
    PrefixExpression,
    ReturnExpression,
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

    fn prefix_expression(token: Token, right: Expression) -> Expression {
        Expression {
            expression_type: ExpressionType::PrefixExpression,
            token: token,
            right: Some(Box::new(right)), 
        }
    }

    fn return_expression(right: Expression) -> Expression {
        Expression {
            expression_type: ExpressionType::ReturnExpression,
            token: Token::keyword_return(),
            right: Some(Box::new(right)),
        }
    }
}

pub fn parse_expression(tokens: &Vec<Token>, from: usize) -> (Option<Expression>, usize) {
    let token = &tokens[from];
    match token.token_type {
        TokenType::Bang => {
            let expression = Expression::prefix_expression(
                tokens[from].clone(),
                Expression::literal_expression(tokens[from+1].clone()
            ));
            return (Some(expression), 2)
        },
        TokenType::String | TokenType::Literal  | TokenType::Number => {
            let expression = Expression::literal_expression(tokens[from].clone());
            return (Some(expression), 1)
        },
        TokenType::Assignment => todo!(),
        TokenType::Plus => todo!(),
        TokenType::Minus => todo!(),
        TokenType::Asterisk => todo!(),
        TokenType::Semicolon => {
            return (None, 1)
        },
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
            let expression = Expression::return_expression(
                Expression::literal_expression(tokens[from+1].clone())
            ); 
            return (Some(expression), 2)
        },
        TokenType::KeywordWhile => todo!(),
        TokenType::KeywordLet => todo!(),
        TokenType::KeywordFn => todo!(),
    }
}

pub fn parse(tokens: &Vec<Token>) ->Vec<Expression> {
    let mut expressions = Vec::new();
    let mut index: usize = 0;

    while index < tokens.len() {
        let (expression, advance) = parse_expression(tokens, index);
        if let Some(exp) = expression {
            expressions.push(exp);
        }
        index = index + advance;
    }
    
    expressions
}