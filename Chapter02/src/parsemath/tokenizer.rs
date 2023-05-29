/// This module reads characters in arithmetic expression and converts them to tokens.
/// The allowed tokens are defined in ast module.
// Standard lib
use std::iter::Peekable;
use std::str::Chars;

//Other internal modules
use super::token::Token;

// Other structs

// Tokenizer struct contains a Peekable iterator on the arithmetic expression
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

// Constructs a new instance of Tokenizer
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

// Implement Iterator trait for Tokenizer struct.
// With this, we can use next() method on tokenier to retrieve the next token from arithmetic expression

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();
        if let Some(char) = next_char {
            match char {
                '0'..='9' => {
                    let mut number = char.to_string();

                    while let Some(next_char) = self.expr.peek() {
                        if next_char.is_numeric() || next_char == &'.' {
                            number.push(self.expr.next()?);
                        } else if next_char == &'(' {
                            return None;
                        } else {
                            break;
                        }
                    }

                    Some(Token::Num(number))
                }
                '+' => Some(Token::Add),
                '-' => Some(Token::Subtract),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                '^' => Some(Token::Caret),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                _ => None,
            }
        } else {
            Some(Token::EOF)
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next(), Some(Token::Num("34.0".to_owned())));
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next(), Some(Token::Num("34.5".to_owned())));
    }
    #[test]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next(), None);
    }
    #[test]
    #[ignore]
    fn test_invalid_number() {
        let mut tokenizer = Tokenizer::new("3.1.1");
        assert_eq!(tokenizer.next(), Some(Token::Num("3.1.1".to_owned())));
    }
}
