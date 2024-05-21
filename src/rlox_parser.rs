use crate::{rlox::{self, Token, TokenType}, rlox_expr::{self, Binary, Expression, Unary}};

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

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expression::Binary(Binary { left: Box::new(expr), right: Box::new(right), operator });
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr: Expression = self.term();

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expression::Binary(Binary { left: Box::new(expr), right: Box::new(right), operator });
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr: Expression = self.factor();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expression::Binary(Binary { left: Box::new(expr), right: Box::new(right), operator });
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr: Expression = self.unary();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expression::Binary(Binary { left: Box::new(expr), right: Box::new(right), operator });
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            Expression::Unary(Unary { operator, right: Box::new(right) })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        Expression::Literal(crate::rlox_expr::Literal { value: rlox::Literal::Identifier("False".to_owned()) })

        // if self.match_token(vec![TokenType::False]) {
        //     return Expression::Literal( rlox_expr::Literal { value: Literal})
        // }
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}