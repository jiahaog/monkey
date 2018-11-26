#![allow(dead_code)]

// TODO consistency of named imports
use ast::Expression;
use ast::Expression::DummyExpression;
use ast::Operator;
use ast::Program;
use ast::Statement;
use ast::Statement::{ExpressionStatement, LetStatement, ReturnStatement};
use lexer::Lexer;
use std::iter::Peekable;
use token::Token;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum ParseErrorExpected {
    ExpectedToken(Token),
    Identifier,
    Expression,
    Assignment,
    PrefixTokenOrExpression,
}

#[derive(Debug, PartialEq)]
struct ParseError {
    expected: ParseErrorExpected,
    received: Option<Token>,
}

struct Parser<'a> {
    // TODO remove the peekable type if it's really unnecessary
    lexer: Peekable<Lexer<'a>>,
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

fn get_precedence(token: &Token) -> Precedence {
    match token {
        Token::Equal => Precedence::Equals,
        Token::NotEqual => Precedence::Equals,
        Token::LessThan => Precedence::LessGreater,
        Token::GreaterThan => Precedence::LessGreater,
        Token::Plus => Precedence::Sum,
        Token::Minus => Precedence::Sum,
        Token::Slash => Precedence::Product,
        Token::Asterisk => Precedence::Product,
        _ => Precedence::Lowest,
    }
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

        println!("oks: {:?} fails: {:?}", values, errors);

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

                let _ = self.next_expression();
                err
            }).and_then(|name| {
                self.next_expression()
                    .map(|expression| LetStatement(name, expression))
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
        self.next_expression().map(|x| ReturnStatement(x))
    }

    fn next_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let result = self
            .parse_expression(Precedence::Lowest)
            .map(|x| ExpressionStatement(x));

        if let Some(Token::Semicolon) = self.lexer.peek() {
            self.lexer.next();
        }

        result
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Expression,
                received: None,
            }).and_then(|token| self.prefix_parse_token(token))
            .and_then(|left| self.parse_next_infix_expression(precedence, left))
    }

    fn parse_next_infix_expression(
        &mut self,
        precedence: Precedence,
        prev: Expression,
    ) -> Result<Expression, ParseError> {
        if let Some(Token::Semicolon) = self.lexer.peek() {
            return Ok(prev);
        }

        match self.lexer.next() {
            None => Ok(prev),
            Some(token) => {
                // recursion here until the condition is broken
                if precedence < get_precedence(&token) {
                    self.infix_parse_token(prev, token)
                        .and_then(|next_exp| self.parse_next_infix_expression(precedence, next_exp))
                } else {
                    Ok(prev)
                }
            }
        }
    }

    fn infix_parse_token(
        &mut self,
        prev: Expression,
        token: Token,
    ) -> Result<Expression, ParseError> {
        let precedence = get_precedence(&token);
        match token {
            Token::Plus => self.parse_infix_expression(precedence, prev, Operator::Plus),
            Token::Minus => self.parse_infix_expression(precedence, prev, Operator::Minus),
            Token::Slash => self.parse_infix_expression(precedence, prev, Operator::Divide),
            Token::Asterisk => self.parse_infix_expression(precedence, prev, Operator::Multiply),
            Token::Equal => self.parse_infix_expression(precedence, prev, Operator::Equal),
            Token::NotEqual => self.parse_infix_expression(precedence, prev, Operator::NotEqual),
            Token::LessThan => self.parse_infix_expression(precedence, prev, Operator::LessThan),
            Token::GreaterThan => {
                self.parse_infix_expression(precedence, prev, Operator::GreaterThan)
            }
            _ => unimplemented!(),
        }
    }

    fn parse_infix_expression(
        &mut self,
        precedence: Precedence,
        left: Expression,
        operator: Operator,
    ) -> Result<Expression, ParseError> {
        self.parse_expression(precedence)
            .map(|next_exp| Expression::InfixExpression {
                operator: operator,
                left: Box::new(left),
                right: Box::new(next_exp),
            })
    }

    fn prefix_parse_token(&mut self, token: Token) -> Result<Expression, ParseError> {
        match token {
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            Token::Int(value) => Ok(Expression::IntegerLiteral(value)),
            Token::Bang => self.parse_prefix_expression(Operator::Not),
            Token::Minus => self.parse_prefix_expression(Operator::Minus),
            Token::Semicolon => Err(ParseError {
                expected: ParseErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Semicolon),
            }),
            _ => unimplemented!(),
        }
    }

    fn parse_prefix_expression(&mut self, operator: Operator) -> Result<Expression, ParseError> {
        self.parse_expression(Precedence::Prefix)
            .map(|next_exp| Expression::PrefixExpression {
                operator: operator,
                right: Box::new(next_exp),
            })
    }

    fn next_expression(&mut self) -> Result<Expression, ParseError> {
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
        Ok(DummyExpression)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
