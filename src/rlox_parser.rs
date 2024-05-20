use crate::{rlox::{Literal, Token}, rlox_expr::{Expression}};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expression {
        return self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        expr
    }

    fn comparison(&mut self) -> Expression {
        self.comparison()
    }

    fn term(&mut self) -> Expression {
        self.term()
    }

    fn factor(&mut self) -> Expression {
        self.unary()
    }

    fn unary(&mut self) -> Expression {
        self.primary()
    }

    fn primary(&mut self) -> Expression {
        Expression::Literal(crate::rlox_expr::Literal { value: Literal::Identifier("False".to_owned()) })
    }
}