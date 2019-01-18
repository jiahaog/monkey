use crate::ast::{CallFunctionExpression, Expression, Function};
use crate::parser::Parser;
use crate::parser::Precedence;
use crate::parser::{ParseError, ParseErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub(super) fn parse_call_expression(
        &mut self,
        function: Expression,
    ) -> Result<Expression, ParseError> {
        self.chomp_call_args(Vec::new())
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
            .map(|args| Expression::Call {
                function: match function {
                    Expression::Identifier(name) => CallFunctionExpression::Identifier(name),
                    Expression::FunctionLiteral(Function { params, body }) => {
                        CallFunctionExpression::Literal(Function {
                            params: params,
                            body: body,
                        })
                    },
                    _ => panic!("The upstream parser should have determined that this can only be a identifier or function literal"),
                },
                arguments: args,
            })
    }

    fn chomp_call_args(
        &mut self,
        mut prev: Vec<Expression>,
    ) -> Result<Vec<Expression>, ParseError> {
        match self.lexer.peek() {
            Some(Token::RParen) => Ok(prev),
            Some(Token::Comma) => {
                self.lexer.next();
                self.chomp_call_args(prev)
            }
            Some(_) => self.next_expression(Precedence::Lowest).and_then(|expr| {
                prev.push(expr);
                self.chomp_call_args(prev)
            }),
            None => Err(ParseError {
                expected: ParseErrorExpected::ClosingParenthesis,
                received: None,
            }),
        }
    }
}
