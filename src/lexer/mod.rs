pub mod token;

use self::token::{Token, TokenKind, TokenSpan};

pub struct Lexer {
    cur_pos: usize,
    read_pos: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            cur_pos: 0,
            read_pos: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        l.consume_char();
        l
    }

    fn consume_char(&mut self) {
        self.cur_pos = self.read_pos;
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_pos];
            self.read_pos += 1;
        }
    }

    fn peek(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_pos];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.consume_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.cur_pos;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.consume_char();
        }
        String::from_utf8_lossy(&self.input[pos..self.cur_pos]).to_string()
    }

    fn read_number(&mut self) -> String {
        let pos = self.cur_pos;
        loop {
            if self.ch.is_ascii_digit() {
                self.consume_char();
                continue;
            }
            if self.ch == b'.' {
                self.consume_char();
                while self.ch.is_ascii_digit() {
                    self.consume_char();
                }
                break;
            }
            break;
        }
        String::from_utf8_lossy(&self.input[pos..self.cur_pos]).to_string()
    }

    fn read_string_literal(&mut self) -> String {
        let pos = self.cur_pos;
        self.consume_char();
        while self.ch != b'"' {
            self.consume_char();
        }
        self.consume_char();
        String::from_utf8_lossy(&self.input[pos..self.cur_pos]).to_string()
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_pos >= self.input.len() {
            return None;
        }

        self.skip_whitespace();
        let start_pos = self.cur_pos;
        let mut consumed = false;
        let kind = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                consumed = true;
                match ident.as_str() {
                    "fn" => TokenKind::Function,
                    "let" => TokenKind::Let,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "return" => TokenKind::Return,
                    _ => TokenKind::Ident(ident),
                }
            }
            b'0'..=b'9' => {
                consumed = true;
                TokenKind::Number(self.read_number())
            }
            b'"' => {
                consumed = true;
                TokenKind::StringLiteral(self.read_string_literal())
            }
            b'=' => match self.peek() {
                b'=' => {
                    self.consume_char();
                    TokenKind::Equal
                }
                _ => TokenKind::Assign,
            },
            b'!' => match self.peek() {
                b'=' => {
                    self.consume_char();
                    TokenKind::NotEqual
                }
                _ => TokenKind::Bang,
            },
            b'>' => match self.peek() {
                b'=' => {
                    self.consume_char();
                    TokenKind::GreaterOrEqual
                }
                _ => TokenKind::GreaterThan,
            },
            b'<' => match self.peek() {
                b'=' => {
                    self.consume_char();
                    TokenKind::LessOrEqual
                }
                _ => TokenKind::LessThan,
            },
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Dash,
            b'*' => TokenKind::Asterisk,
            b'/' => TokenKind::Slash,
            b',' => TokenKind::Comma,
            b';' => TokenKind::Semicolon,
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'{' => TokenKind::LSquirly,
            b'}' => TokenKind::RSquirly,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            0 => TokenKind::EOF,
            _ => TokenKind::Illegal,
        };

        if !consumed {
            self.consume_char();
        }

        let end_pos = self.cur_pos;
        let span = TokenSpan::new(
            start_pos,
            end_pos,
            String::from_utf8_lossy(&self.input[start_pos..end_pos]).to_string(),
        );

        Some(Token::new(kind, span))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::fs;

    #[test]
    fn lex_kind() -> Result<()> {
        let input = fs::read_to_string("./example.jsh")?;
        let mut lexer = Lexer::new(input);
        let kinds: Vec<TokenKind> = vec![
            TokenKind::Let,
            TokenKind::Ident("five".to_string()),
            TokenKind::Assign,
            TokenKind::Number("5".to_string()),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident("ten".to_string()),
            TokenKind::Assign,
            TokenKind::Number("10".to_string()),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident("fraction".to_string()),
            TokenKind::Assign,
            TokenKind::Number("110.598".to_string()),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident("add".to_string()),
            TokenKind::Assign,
            TokenKind::Function,
            TokenKind::LParen,
            TokenKind::Ident("x".to_string()),
            TokenKind::Comma,
            TokenKind::Ident("y".to_string()),
            TokenKind::RParen,
            TokenKind::LSquirly,
            TokenKind::Ident("x".to_string()),
            TokenKind::Plus,
            TokenKind::Ident("y".to_string()),
            TokenKind::Semicolon,
            TokenKind::RSquirly,
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident("result".to_string()),
            TokenKind::Assign,
            TokenKind::Ident("add".to_string()),
            TokenKind::LParen,
            TokenKind::Ident("five".to_string()),
            TokenKind::Comma,
            TokenKind::Ident("ten".to_string()),
            TokenKind::RParen,
            TokenKind::Semicolon,
            TokenKind::Bang,
            TokenKind::Dash,
            TokenKind::Slash,
            TokenKind::Asterisk,
            TokenKind::Number("5".to_string()),
            TokenKind::Semicolon,
            TokenKind::Number("5".to_string()),
            TokenKind::LessThan,
            TokenKind::Number("10".to_string()),
            TokenKind::GreaterThan,
            TokenKind::Number("5".to_string()),
            TokenKind::Semicolon,
            TokenKind::If,
            TokenKind::LParen,
            TokenKind::Number("5".to_string()),
            TokenKind::LessThan,
            TokenKind::Number("10".to_string()),
            TokenKind::RParen,
            TokenKind::LSquirly,
            TokenKind::Return,
            TokenKind::True,
            TokenKind::Semicolon,
            TokenKind::RSquirly,
            TokenKind::Else,
            TokenKind::LSquirly,
            TokenKind::Return,
            TokenKind::False,
            TokenKind::Semicolon,
            TokenKind::RSquirly,
            TokenKind::Number("10".to_string()),
            TokenKind::Equal,
            TokenKind::Number("10".to_string()),
            TokenKind::Semicolon,
            TokenKind::Number("10".to_string()),
            TokenKind::NotEqual,
            TokenKind::Number("9".to_string()),
            TokenKind::Semicolon,
            TokenKind::StringLiteral("\"test\"".to_string()),
            TokenKind::Semicolon,
            TokenKind::EOF,
        ];
        for kind in kinds {
            let next = lexer.next().unwrap().kind;
            assert_eq!(kind, next);
        }

        Ok(())
    }

    #[test]
    fn lex_token() -> Result<()> {
        let input = fs::read_to_string("./examples/example.jsh")?;
        let mut lexer = Lexer::new(input);
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Let, TokenSpan::new(0, 3, "let".into())),
            Token::new(
                TokenKind::Ident("five".into()),
                TokenSpan::new(4, 8, "five".into()),
            ),
            Token::new(TokenKind::Assign, TokenSpan::new(9, 10, "=".into())),
            Token::new(
                TokenKind::Number("5".into()),
                TokenSpan::new(11, 12, "5".into()),
            ),
            Token::new(TokenKind::Semicolon, TokenSpan::new(12, 13, ";".into())),
            Token::new(TokenKind::Let, TokenSpan::new(14, 17, "let".into())),
        ];
        for token in tokens {
            let next = lexer.next().unwrap();
            assert_eq!(token, next);
        }

        Ok(())
    }
}
