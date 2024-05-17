use crate::rlox::Token;

pub trait TestExpr {
    fn accept<R>(&mut self, visitor: &impl Visitor<R>) -> R;
}

pub struct TestVisitor;

impl TestExpr for Binary {
    fn accept<R>(&mut self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_binary(&self)
    }
}


pub trait Expr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;
}


macro_rules! ast {
    ($expression:ident $(, $name:ident: $type:ty)+) => {
        pub struct $expression {
            $(pub $name: $type),+
        }
        //impl Expr for $expression {
            // fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {

            // }
        //}
    };
}

macro_rules! visitor {
    ($($name:ident, $type:ty,)+) => {
        pub trait Visitor<T> {
            $(fn $name(&self, expression: &$type) -> T;)+
        }
    };
}

macro_rules! impl_expr {
    ($name:ident, $type:ty) => {
        impl Expr for $type {
            fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
                visitor.$name(self)
            }
        }
    };
}

ast!(Binary, left: Box<Expression>, right: Box<Expression>, operator: Token);
ast!(Unary, operator: Token, right: Box<Expression>);
ast!(Grouping, expression: Box<Expression>);
ast!(Literal, value: crate::rlox::Literal);

pub enum Expression {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

pub fn accept_visitor<R>(expression: &Expression, visitor: &impl Visitor<R>) -> R {
    match expression {
        Expression::Binary(binary) => binary.accept(visitor),
        Expression::Grouping(grouping) => grouping.accept(visitor),
        Expression::Literal(literal) => literal.accept(visitor),
        Expression::Unary(unary) => unary.accept(visitor),
    }
}

impl_expr!(visit_binary, Binary);
impl_expr!(visit_unary, Unary);
impl_expr!(visit_grouping, Grouping);
impl_expr!(visit_literal, Literal);

visitor!(
    visit_binary, Binary,
    visit_unary, Unary,
    visit_grouping, Grouping,
    visit_literal, Literal,
);

macro_rules! match_expression {
    ($expression:ident, $binary:block, $grouping:block, $literal:block, $unary:block) => {
        match $expression {
            Expression::Binary(binary) => $binary,
            Expression::Grouping(grouping) => $grouping
            Expression::Literal(literal) => $literal
            Expression::Unary(unary) => $unary
        }
    };
}

pub struct Printer {}

impl Printer {
    pub fn print(&self, expression: &Expression) -> String {
        match expression {
            Expression::Binary(binary) => self.visit_binary(binary),
            Expression::Grouping(grouping) => self.visit_grouping(grouping),
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Unary(unary) => self.visit_unary(unary),
        }
        // match_expression!(expression,
        //     { self.visit_binary(binary) },
        //     { self.visit_grouping(grouping) },
        //     { self.visit_literal(literal) },
        //     { self.visit_unary(unary) })
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, expression: &Binary) -> String {
        format!("({} {} {})", expression.operator.lexeme, accept_visitor(expression.left.as_ref(), self), accept_visitor(expression.right.as_ref(), self))
    }

    fn visit_grouping(&self, expression: &Grouping) -> String {
        format!("(group {})", accept_visitor(expression.expression.as_ref(), self))
    }

    fn visit_literal(&self, expression: &Literal) -> String {
        match &expression.value {
            crate::rlox::Literal::None => "nil".to_owned(),
            crate::rlox::Literal::Identifier(identifier) => identifier.to_owned(),
            crate::rlox::Literal::String(string) => string.to_owned(),
            crate::rlox::Literal::Number(number) => format!("{}", number),
        }
    }

    fn visit_unary(&self, expression: &Unary) -> String {
        format!("({} {})", expression.operator.lexeme, accept_visitor(expression.right.as_ref(), self))

    }
}