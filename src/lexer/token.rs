#[derive(Debug, PartialEq)]
pub enum TokenKind {
    EOF,
    Ident(String),
    Number(String),
    StringLiteral(String),
    Illegal,
    Equal,
    Assign,
    Bang,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    Plus,
    Dash,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LSquirly,
    RSquirly,
    LBracket,
    RBracket,
    Function,
    Let,
    If,
    Else,
    True,
    False,
    Return,
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    start: usize,
    end: usize,
    lexeme: String,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize, lexeme: String) -> Self {
        Self { start, end, lexeme }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    span: TokenSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TokenSpan) -> Self {
        Self { kind, span }
    }
}
