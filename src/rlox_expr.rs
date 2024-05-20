use crate::rlox::Token;

pub trait Expr {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R;
}

macro_rules! define_expression {
    ($expression:ident $(, $name:ident: $type:ty)+) => {
        pub struct $expression {
            $(pub $name: $type),+
        }
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

define_expression!(Binary, left: Box<Expression>, right: Box<Expression>, operator: Token);
define_expression!(Unary, operator: Token, right: Box<Expression>);
define_expression!(Grouping, expression: Box<Expression>);
define_expression!(Literal, value: crate::rlox::Literal);

pub enum Expression {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

impl Expression {
    pub fn visit<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Unary(unary) => visitor.visit_unary(unary),
        }
    }

    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
            Expression::Binary(binary) => binary.accept(visitor),
            Expression::Grouping(grouping) => grouping.accept(visitor),
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
        }
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

pub struct Printer {}

impl Printer {
    pub fn print(&self, expression: &Expression) -> String {
        expression.visit(self)
    }
}

impl Visitor<String> for Printer {
    fn visit_binary(&self, expression: &Binary) -> String {
        format!("({} {} {})", expression.operator.lexeme, expression.left.accept(self), expression.right.accept(self))
    }

    fn visit_grouping(&self, expression: &Grouping) -> String {
        format!("(group {})", expression.expression.accept(self))

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
        format!("({} {})", expression.operator.lexeme, expression.right.accept(self))
    }
}