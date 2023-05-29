/// This contains enum for list of Tokens, and handles Operator precedence rules.

// List of valid tokens that can be constructed from arithmetic expression by Tokenizer

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(String),
    EOF,
}

/// Order of operators as per operator precedence rules (low to high)
/// Defines all the OperPrec levels, from lowest to highest.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        match self {
            Token::Add | Token::Subtract => OperPrec::AddSub,
            Token::Multiply | Token::Divide => OperPrec::MulDiv,
            Token::Caret => OperPrec::Power,

            _ => OperPrec::DefaultZero,
        }
    }
}
