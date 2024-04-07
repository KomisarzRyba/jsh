use super::expression;

pub trait Statement {
    fn stmt(&self);
}

pub struct BlockStatement {
    body: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn push(&mut self, value: Box<dyn Statement>) {
        self.body.push(value)
    }
}

impl Statement for BlockStatement {
    fn stmt(&self) {
        todo!()
    }
}

struct ExpressionStatement {
    expression: Box<dyn expression::Expression>,
}

impl Statement for ExpressionStatement {
    fn stmt(&self) {
        todo!()
    }
}
