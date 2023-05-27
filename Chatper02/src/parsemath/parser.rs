use std::error;
use std::fmt;
use std::num;

use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

//Structs and constants

// Parser struct
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Public methods of Parser

impl<'a> Parser<'a> {
    // Create a new instance of Parser
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = lexer
            .next()
            .ok_or_else(|| ParseError::InvalidOperator("Invalid character".into()))?;
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    // Take an arithmetic expression as input and return an AST

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.generate_ast(OperPrec::DefaultZero)
    }
}

// Private methods of Parser

impl<'a> Parser<'a> {
    // Retrieve the next token from arithmetic expression and set it to current_token field in Parser struct
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        self.current_token = self
            .tokenizer
            .next()
            .ok_or_else(|| ParseError::InvalidOperator("Invalid character".into()))?;
        Ok(())
    }

    // Main workhorse method that is called recursively

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    // Construct AST node for numbers, taking into account negative prefixes while handling parenthesis

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i.parse::<f64>()?))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }

                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    // Check for balancing parenthesis

    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }

    // Construct Operator AST nodes

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

// Custom error handler for Parser
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "Error unable to parse {}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "Error invalid operator {}", e),
            self::ParseError::InvalidNumber(e) => write!(f, "Error parse number {}", e),
        }
    }
}

impl From<Box<dyn error::Error>> for ParseError {
    fn from(_e: Box<dyn error::Error>) -> Self {
        Self::UnableToParse("Unable to parse".into())
    }
}

impl From<num::ParseFloatError> for ParseError {
    fn from(e: num::ParseFloatError) -> Self {
        Self::InvalidNumber(e.to_string())
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::{Add, Number};
    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
