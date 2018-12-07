#![allow(dead_code)]

mod error;
mod precedence;
#[cfg(test)]
mod tests;

use self::error::{ParseError, ParseErrorExpected};
use self::precedence::Precedence;
use ast::{Expression, Operator, Program, Statement};
use lexer::Lexer;
use std::iter::Peekable;
use token::Token;

struct Parser<'a> {
    // TODO remove the peekable type if it's really unnecessary
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    fn parse(self) -> Result<Program, Vec<ParseError>> {
        let (oks, fails): (Vec<_>, Vec<_>) = self.partition(Result::is_ok);
        let values = oks.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = fails.into_iter().map(Result::unwrap_err).collect();

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(Program::new(values))
        }
    }

    fn next_statement(&mut self) -> Option<Result<Statement, ParseError>> {
        match self.lexer.peek() {
            None => None,
            Some(Token::Let) => {
                self.lexer.next();
                Some(self.next_let_statement())
            }
            Some(Token::Return) => {
                self.lexer.next();
                Some(self.next_return_statement())
            }
            _ => Some(self.next_expression_statement()),
        }
    }

    fn next_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_let_statement_identifier()
            .and_then(|name| self.next_let_statement_assign().map(|_| name))
            .map_err(|err| {
                // Increment the iterator until the semicolon, so that the next call to
                // next_let_statement will continue with the next line.
                // We do this before the success case, because we don't want to call
                // next_expression() twice

                let _ = self.next_expression_dummy();
                err
            }).and_then(|name| {
                self.next_expression_dummy()
                    .map(|expression| Statement::Let(name, expression))
            })
    }

    fn next_let_statement_identifier(&mut self) -> Result<String, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Identifier,
                received: None,
            }).and_then(|token| match token {
                Token::Identifier(name) => Ok(name),
                unexpected => Err(ParseError {
                    expected: ParseErrorExpected::Identifier,
                    received: Some(unexpected),
                }),
            })
    }

    fn next_let_statement_assign(&mut self) -> Result<Token, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Assignment,
                received: None,
            }).and_then(|token| match token {
                Token::Assign => Ok(token),
                unexpected => Err(ParseError {
                    expected: ParseErrorExpected::Assignment,
                    received: Some(unexpected),
                }),
            })
    }

    fn next_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_expression_dummy().map(|x| Statement::Return(x))
    }

    fn next_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let result = self
            .next_expression(Precedence::Lowest)
            .map(|x| Statement::Expression(x));

        if let Some(Token::Semicolon) = self.lexer.peek() {
            self.lexer.next();
        }

        result
    }

    fn next_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Expression,
                received: None,
            }).and_then(|token| self.next_prefix_expression(token))
            .and_then(|left| self.next_infix_expression(precedence, left))
    }

    fn next_infix_expression(
        &mut self,
        precedence: Precedence,
        prev: Expression,
    ) -> Result<Expression, ParseError> {
        if let Some(Token::Semicolon) = self.lexer.peek() {
            return Ok(prev);
        }

        // NLL should make this less ugly
        if let Some(true) = self
            .lexer
            .peek()
            .map(|token| precedence < Precedence::from_token(token))
        {
            let x = self.lexer.next().unwrap();
            self.parse_infix_from_token(prev, x)
                .and_then(|next_exp| self.next_infix_expression(precedence, next_exp))
        } else {
            Ok(prev)
        }
    }

    fn parse_infix_from_token(
        &mut self,
        prev: Expression,
        token: Token,
    ) -> Result<Expression, ParseError> {
        let precedence = Precedence::from_token(&token);
        match token {
            Token::Plus => self.parse_infix_expr(precedence, prev, Operator::Plus),
            Token::Minus => self.parse_infix_expr(precedence, prev, Operator::Minus),
            Token::Slash => self.parse_infix_expr(precedence, prev, Operator::Divide),
            Token::Asterisk => self.parse_infix_expr(precedence, prev, Operator::Multiply),
            Token::Equal => self.parse_infix_expr(precedence, prev, Operator::Equal),
            Token::NotEqual => self.parse_infix_expr(precedence, prev, Operator::NotEqual),
            Token::LessThan => self.parse_infix_expr(precedence, prev, Operator::LessThan),
            Token::GreaterThan => self.parse_infix_expr(precedence, prev, Operator::GreaterThan),
            _ => unimplemented!(),
        }
    }

    fn parse_infix_expr(
        &mut self,
        precedence: Precedence,
        left: Expression,
        operator: Operator,
    ) -> Result<Expression, ParseError> {
        self.next_expression(precedence)
            .map(|next_exp| Expression::Infix {
                operator,
                left: Box::new(left),
                right: Box::new(next_exp),
            })
    }

    fn next_prefix_expression(&mut self, token: Token) -> Result<Expression, ParseError> {
        match token {
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            Token::Int(value) => Ok(Expression::IntegerLiteral(value)),
            Token::Bang => self.parse_prefix_expr(Operator::Not),
            Token::Minus => self.parse_prefix_expr(Operator::Minus),
            Token::Semicolon => Err(ParseError {
                expected: ParseErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Semicolon),
            }),
            Token::True => Ok(Expression::Boolean(true)),
            Token::False => Ok(Expression::Boolean(false)),
            Token::LParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            x => unimplemented!("Token: {:?}", x),
        }
    }

    fn parse_prefix_expr(&mut self, operator: Operator) -> Result<Expression, ParseError> {
        self.next_expression(Precedence::Prefix)
            .map(|next_exp| Expression::Prefix {
                operator,
                right: Box::new(next_exp),
            })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_expression(Precedence::Lowest)
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RParen) => {
                    self.lexer.next();
                    Ok(expr)
                }
                _ => Err(ParseError {
                    expected: ParseErrorExpected::ClosingParenthesis,
                    received: self.lexer.next(),
                }),
            })
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_if_expression_conditional()
            .and_then(|conditional| {
                self.parse_if_expression_consequence()
                    .map(|consequence| (conditional, consequence))
            }).and_then(|(conditional, consequence)| {
                self.parse_if_expression_alternative()
                    .map(|alternative| (conditional, consequence, alternative))
            }).map(|(condition, consequence, alternative)| Expression::If {
                condition: Box::new(condition),
                consequence: consequence,
                alternative: alternative,
            })
    }

    fn parse_if_expression_conditional(&mut self) -> Result<Expression, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::ParenthesisForIfCondition,
                received: None,
            }).and_then(|token| match token {
                Token::LParen => self.next_expression(Precedence::Lowest),
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParenthesisForIfCondition,
                    received: Some(x),
                }),
            }).and_then(|expr| match self.lexer.peek() {
                Some(Token::RParen) => {
                    self.lexer.next();
                    Ok(expr)
                }
                _ => Err(ParseError {
                    expected: ParseErrorExpected::ClosingParenthesis,
                    received: self.lexer.next(),
                }),
            })
    }

    fn parse_if_expression_consequence(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::ParenthesisForIfCondition,
                received: None,
            }).and_then(|token| match token {
                Token::LBrace => self.parse_block_statement(),
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParenthesisForIfCondition,
                    received: Some(x),
                }),
            })
    }

    fn parse_if_expression_alternative(&mut self) -> Result<Vec<Statement>, ParseError> {
        // Refactor this with NLL
        if let Some(Token::Else) = self.lexer.peek() {
            self.lexer.next(); // consume the else

            self.lexer
                .next()
                .ok_or(ParseError {
                    expected: ParseErrorExpected::ParenthesisForIfCondition,
                    received: None,
                }).and_then(|token| match token {
                    Token::LBrace => self.parse_block_statement(),
                    x => Err(ParseError {
                        expected: ParseErrorExpected::ParenthesisForIfCondition,
                        received: Some(x),
                    }),
                })
        } else {
            Ok(Vec::new())
        }
    }

    fn parse_block_statement(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();

        loop {
            if let None = self.lexer.peek() {
                return Ok(statements);
            }

            if let Some(Token::RBrace) = self.lexer.peek() {
                self.lexer.next();
                return Ok(statements);
            }

            match self.next_statement().unwrap() {
                Ok(statement) => statements.push(statement),
                Err(x) => return Err(x),
            }
        }
    }

    fn next_expression_dummy(&mut self) -> Result<Expression, ParseError> {
        // TODO temporary hack to chomp up stuff until the semicolon to make
        // tests pass
        loop {
            let current = self.lexer.next();
            if let Some(token) = current {
                if let Token::Semicolon = token {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(Expression::DummyExpression)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
