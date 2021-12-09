use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
    LiteralExpression,
    PrefixExpression,
    InfixExpression,
    ReturnExpression,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression_type: ExpressionType,
    pub token: Token,
    pub left: Option<Box<Expression>>,
    pub right: Option<Box<Expression>>,
}

impl Expression {
    fn literal_expression(token: Token) -> Expression {
        Expression {
            expression_type: ExpressionType::LiteralExpression,
            token: token,
            left: None,
            right: None,
        }
    }

    fn prefix_expression(token: Token, right: Expression) -> Expression {
        Expression {
            expression_type: ExpressionType::PrefixExpression,
            token: token,
            left: None,
            right: Some(Box::new(right)),
        }
    }

    fn infix_expression(token: Token, left: Expression, right: Expression) -> Expression {
        Expression {
            expression_type: ExpressionType::InfixExpression,
            token: token,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn return_expression(right: Expression) -> Expression {
        Expression {
            expression_type: ExpressionType::ReturnExpression,
            token: Token::keyword_return(),
            left: None,
            right: Some(Box::new(right)),
        }
    }

    pub fn right_side(&self) -> &Expression {
        self.right.as_ref().unwrap().as_ref()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    expressions: Vec<Expression>,
    token_index: usize,
    parsing_infix: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            expressions: Vec::new(),
            token_index: 0,
            parsing_infix: false,
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

    fn parse_expression(&self, from: usize) -> (Option<Expression>, usize) {
        let token = &self.tokens[from];
        match token.token_type {
            TokenType::Bang => {
                let (rhs, adv) = self.parse_expression(from+1);
                let expression = Expression::prefix_expression(
                    self.tokens[from].clone(),
                    rhs.unwrap()
                );
                return (Some(expression), 1 + adv);
            }
            TokenType::String | TokenType::Literal | TokenType::Number => {
                let expression = Expression::literal_expression(self.tokens[from].clone());
                return (Some(expression), 1);
            }
            TokenType::Assignment => todo!(),
            TokenType::Plus => todo!(),
                /*let (rhs, adv) = parse_expression(tokens, from+1);
                let (lhs, _) = parse_expression(tokens, from - 1);
    
                let expression = Expression::prefix_expression(
                    tokens[from].clone(),
                    rhs.unwrap()
                );
                return (Some(expression), 1 + adv);
            },*/
            TokenType::Minus => todo!(),
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
                let (right_expression, positions) = self.parse_expression(self.token_index + 1);
                let expression = Expression::return_expression(right_expression.unwrap());
                return (Some(expression), 2 + positions);
            }
            TokenType::KeywordWhile => todo!(),
            TokenType::KeywordLet => todo!(),
            TokenType::KeywordFn => todo!(),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut parser = Parser::new(tokens);
    parser.parse();
    parser.expressions
}
