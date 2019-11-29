use crate::ast::Expression;
use crate::parser::Parser;
use crate::parser::Precedence;
use crate::parser::{ParseError, ParseErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub(super) fn parse_list_expression(&mut self) -> Result<Expression, ParseError> {
        self.chomp_list_values(Vec::new())
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RBracket) => {
                    self.lexer.next();
                    Ok(Expression::ListLiteral(expr))
                }
                _ => Err(ParseError {
                    expected: ParseErrorExpected::ClosingBracket,
                    received: self.lexer.next(),
                }),
            })
    }

    fn chomp_list_values(
        &mut self,
        mut prev: Vec<Expression>,
    ) -> Result<Vec<Expression>, ParseError> {
        match self.lexer.peek() {
            Some(Token::RBracket) => Ok(prev),
            Some(Token::Comma) => {
                self.lexer.next();
                self.chomp_list_values(prev)
            }
            _ => {
                let expr = self.next_expression(Precedence::Lowest)?;
                prev.push(expr);
                self.chomp_list_values(prev)
            }
        }
    }
}
