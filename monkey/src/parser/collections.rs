use crate::ast::Expression;
use crate::parser::Parser;
use crate::parser::Precedence;
use crate::parser::{ParseError, ParseErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub(super) fn parse_array_expression(&mut self) -> Result<Expression, ParseError> {
        self.chomp_array_values(Vec::new())
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RBracket) => {
                    self.lexer.next();
                    Ok(Expression::ArrayLiteral(expr))
                }
                _ => Err(ParseError {
                    expected: ParseErrorExpected::ClosingBracket,
                    received: self.lexer.next(),
                }),
            })
    }

    fn chomp_array_values(
        &mut self,
        mut prev: Vec<Expression>,
    ) -> Result<Vec<Expression>, ParseError> {
        match self.lexer.peek() {
            Some(Token::RBracket) => Ok(prev),
            Some(Token::Comma) => {
                self.lexer.next();
                self.chomp_array_values(prev)
            }
            _ => {
                let expr = self.next_expression(Precedence::Lowest)?;
                prev.push(expr);
                self.chomp_array_values(prev)
            }
        }
    }
}
