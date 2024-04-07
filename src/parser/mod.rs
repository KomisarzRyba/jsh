mod error;
mod lookup;

use crate::{
    ast::{
        expression,
        statement::{self, BlockStatement},
    },
    lexer::{self, token::Token},
};
use error::Result;

use self::error::Error;

struct Parser {
    tokens: Vec<Token>,
    cur_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cur_pos: 0 }
    }

    pub fn parse(&mut self) -> BlockStatement {
        let mut stmt = BlockStatement::new();
        while let Some(token) = self.next() {
            stmt.push(self.parse_statement())
        }
        stmt
    }

    fn cur_token(&self) -> Option<Token> {
        if self.cur_pos >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.cur_pos])
    }

    fn consume_token(&mut self) -> Option<Token> {
        let t = self.cur_token();
        self.cur_pos += 1;
        t
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_token()
    }
}

trait StatementParser {
    fn parse_statement(&self) -> Box<dyn statement::Statement>;
}

impl StatementParser for Parser {
    fn parse_statement(&self) -> Box<dyn statement::Statement> {
        todo!()
    }
}

trait ExpressionParser {
    fn parse_expression(&self, binding_power: u8) -> Result<Box<dyn expression::Expression>>;
    fn parse_primary_expression(&self) -> Result<Box<dyn expression::Expression>>;
    fn parse_binary_expression(
        &self,
        left: impl expression::Expression,
        binding_power: u8,
    ) -> Result<Box<dyn expression::Expression>>;
}

impl ExpressionParser for Parser {
    fn parse_expression(&self, binding_power: u8) -> Result<Box<dyn expression::Expression>> {
        let token = self.cur_token().ok_or(Error::NoToken)?;
        todo!()
    }

    fn parse_primary_expression(&self) -> Result<Box<dyn expression::Expression>> {
        let token = self.consume_token().ok_or(Error::NoToken)?;
        match token.kind {
            lexer::token::TokenKind::Number(n) => Ok(Box::new(expression::NumberExpression {
                value: n.parse().unwrap(),
            })),
            lexer::token::TokenKind::StringLiteral(s) => {
                Ok(Box::new(expression::StringExpression { value: s }))
            }
            lexer::token::TokenKind::Ident(i) => {
                Ok(Box::new(expression::SymbolExpression { value: i }))
            }
            kind => Err(Error::InvalidTokenKind(kind)),
        }
    }

    fn parse_binary_expression(
        &self,
        left: impl expression::Expression,
        binding_power: u8,
    ) -> Result<Box<dyn expression::Expression>> {
        let operator = self.consume_token().ok_or(Error::NoToken)?;
        let right = self.parse_expression(binding_power);
        Ok(Box::new(expression::BinaryExpression::new(
            Box::new(left),
            operator,
            right?,
        )))
    }
}
