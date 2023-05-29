/// This program contains list of valid AST nodes that can be constructed and also evaluates an AST to compute a value
// Standard lib
use std::error;

//structs

// List of allowed AST nodes that can be constructed by Parser
// Tokens can be arithmetic operators or a Number
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

impl Node {
    // Given an AST, calculate the numeric value.
    pub fn eval(&self) -> Result<f64, Box<dyn error::Error>> {
        match self {
            Node::Number(i) => Ok(*i),
            Node::Add(expr1, expr2) => Ok(expr1.eval()? + expr2.eval()?),
            Node::Subtract(expr1, expr2) => Ok(expr1.eval()? - expr2.eval()?),
            Node::Multiply(expr1, expr2) => Ok(expr1.eval()? * expr2.eval()?),
            Node::Divide(expr1, expr2) => Ok(expr1.eval()? / expr2.eval()?),
            Node::Negative(expr1) => Ok(-(expr1.eval()?)),
            Node::Caret(expr1, expr2) => Ok(expr1.eval()?.powf(expr2.eval()?)),
        }
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_expr1() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = ast.eval().unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = ast.eval().unwrap();
        assert_eq!(value, 3.75);
    }
}
