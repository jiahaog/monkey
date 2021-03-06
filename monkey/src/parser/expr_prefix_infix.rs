use crate::ast::{Expression, Operator};
use crate::parser::Parser;
use crate::parser::Precedence;
use crate::parser::{Error, ErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub fn next_expression(&mut self, precedence: Precedence) -> Result<Expression, Error> {
        self.lexer
            .next()
            .ok_or(Error {
                expected: ErrorExpected::Expression,
                received: None,
            })
            .and_then(|token| self.next_prefix_expression(token))
            .and_then(|left| self.next_infix_expression(precedence, left))
    }

    fn next_infix_expression(
        &mut self,
        precedence: Precedence,
        prev: Expression,
    ) -> Result<Expression, Error> {
        match self.lexer.peek() {
            Some(Token::Semicolon) => Ok(prev),
            Some(token) if precedence < token.into() => {
                let x = self.lexer.next().unwrap();
                self.parse_infix_from_token(prev, x)
                    .and_then(|next_exp| self.next_infix_expression(precedence, next_exp))
            }
            _ => Ok(prev),
        }
    }

    fn parse_infix_from_token(
        &mut self,
        prev: Expression,
        token: Token,
    ) -> Result<Expression, Error> {
        let precedence = (&token).into();
        match token {
            Token::Plus => self.parse_infix_expr(precedence, prev, Operator::Plus),
            Token::Minus => self.parse_infix_expr(precedence, prev, Operator::Minus),
            Token::Slash => self.parse_infix_expr(precedence, prev, Operator::Divide),
            Token::Asterisk => self.parse_infix_expr(precedence, prev, Operator::Multiply),
            Token::Equal => self.parse_infix_expr(precedence, prev, Operator::Equal),
            Token::NotEqual => self.parse_infix_expr(precedence, prev, Operator::NotEqual),
            Token::LessThan => self.parse_infix_expr(precedence, prev, Operator::LessThan),
            Token::GreaterThan => self.parse_infix_expr(precedence, prev, Operator::GreaterThan),
            Token::LParen => self.parse_call_expression(prev),
            Token::LBracket => self.parse_index_expression(prev),
            token => Err(Error {
                expected: ErrorExpected::Expression,
                received: Some(token),
            }),
        }
    }

    fn parse_index_expression(&mut self, prev: Expression) -> Result<Expression, Error> {
        self.next_expression(Precedence::Lowest)
            // If no next expression can be successfully parsed, replace the error message.
            .map_err(
                |Error {
                     expected: _,
                     received,
                 }| Error {
                    expected: ErrorExpected::SingleIndex,
                    received: received,
                },
            )
            .map(|next_expr| Expression::Index {
                left: prev.into(),
                index: next_expr.into(),
            })
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RBracket) => {
                    self.lexer.next();
                    Ok(expr)
                }
                _ => Err(Error {
                    expected: ErrorExpected::SingleIndex,
                    received: self.lexer.next(),
                }),
            })
    }

    fn parse_infix_expr(
        &mut self,
        precedence: Precedence,
        left: Expression,
        operator: Operator,
    ) -> Result<Expression, Error> {
        self.next_expression(precedence)
            .map(|next_expr| Expression::Infix {
                operator,
                left: Box::new(left),
                right: Box::new(next_expr),
            })
    }

    fn next_prefix_expression(&mut self, token: Token) -> Result<Expression, Error> {
        match token {
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            Token::Int(value) => Ok(Expression::IntegerLiteral(value)),
            Token::Str(value) => Ok(Expression::StringLiteral(value)),
            Token::Bang => self.parse_prefix_expr(Operator::Not),
            Token::Minus => self.parse_prefix_expr(Operator::Minus),
            Token::Semicolon => Err(Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Semicolon),
            }),
            Token::True => Ok(Expression::Boolean(true)),
            Token::False => Ok(Expression::Boolean(false)),
            Token::LParen => self.parse_grouped_expression(),
            Token::LBracket => self.parse_list_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_expression(),
            Token::Return => Err(Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Return),
            }),
            token => Err(Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(token),
            }),
        }
    }

    fn parse_prefix_expr(&mut self, operator: Operator) -> Result<Expression, Error> {
        self.next_expression(Precedence::Prefix)
            .map(|next_exp| Expression::Prefix {
                operator,
                right: Box::new(next_exp),
            })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, Error> {
        self.next_expression(Precedence::Lowest)
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RParen) => {
                    self.lexer.next();
                    Ok(expr)
                }
                _ => Err(Error {
                    expected: ErrorExpected::ClosingParenthesis,
                    received: self.lexer.next(),
                }),
            })
    }
}
