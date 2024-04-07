use crate::lexer;

pub trait Expression {
    fn expr(&self);
}

struct LiteralExpression<V> {
    pub value: V,
}

impl<V> LiteralExpression<V> {
    fn new(value: V) -> Self {
        Self { value }
    }
}

impl<V> Expression for LiteralExpression<V> {
    fn expr(&self) {
        todo!()
    }
}

pub type NumberExpression = LiteralExpression<f64>;
pub type StringExpression = LiteralExpression<String>;
pub type SymbolExpression = LiteralExpression<String>;

pub struct BinaryExpression {
    left: Box<dyn Expression>,
    operator: lexer::token::Token,
    right: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn new(
        left: Box<dyn Expression>,
        operator: lexer::token::Token,
        right: Box<dyn Expression>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Expression for BinaryExpression {
    fn expr(&self) {
        todo!()
    }
}
