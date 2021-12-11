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

    pub fn left_side(&self) -> &Expression {
        self.left.as_ref().unwrap().as_ref()
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

    fn peek_next(&self, from: usize) -> Option<Token> {
        if (from + 1) < self.tokens.len() {
            Some(self.tokens[from+1].clone())
        } else {
            None
        }
    }

    fn parse_number(&self, from: usize) -> Expression {
        Expression::literal_expression(self.tokens[from].clone())
    }

    fn parse_expression(&mut self, from: usize) -> (Option<Expression>, usize) {
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
                let next_token = self.peek_next(from).unwrap();
                if next_token.token_type == TokenType::Plus {
                    let (infix, adv) = self.parse_expression(from + 1);
                    return (infix, 1 + adv)
                }
                return (Some(self.parse_number(from)), 1);
            }
            TokenType::Assignment => todo!(),
            TokenType::Plus => {
                self.parsing_infix = true;
                let lhs = self.parse_number(from - 1);
                let (rhs, adv) = self.parse_expression(from + 1);
    
                let expression = Expression::infix_expression(
                    self.tokens[from].clone(),
                    lhs,
                    rhs.unwrap()
                );
                self.parsing_infix = false;
                return (Some(expression), 1 + adv);
            },
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
                return (Some(expression), 1 + positions);
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
