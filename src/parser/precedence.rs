use token::Token;

#[derive(PartialOrd, PartialEq)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    // TODO
    // Call,
}

impl Precedence {
    pub fn from_token(token: &Token) -> Precedence {
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
}