#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    BadToken,
    EOF,
    Whitespace,
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    cur_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cur_pos: 0 }
    }

    fn cur_char(&self) -> char {
        self.input.chars().nth(self.cur_pos).unwrap()
    }

    fn consume(&mut self) -> Option<char> {
        if self.cur_pos >= self.input.len() {
            return None;
        }
        let c = self.cur_char();
        self.cur_pos += 1;
        Some(c)
    }

    fn consume_number(&mut self) -> i64 {
        let mut num = 0i64;
        while let Some(c) = self.consume() {
            let c = c.to_digit(10);
            match c {
                Some(d) => {
                    num = num * 10 + d as i64;
                }
                None => {
                    self.cur_pos -= 1;
                    break;
                }
            }
        }
        num
    }

    fn next(&mut self) -> Option<Token> {
        if self.cur_pos > self.input.len() {
            return None;
        }
        if self.cur_pos == self.input.len() {
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, '\0'.to_string()),
            ));
        }

        let start = self.cur_pos;
        let c = self.cur_char();
        let mut kind = TokenKind::BadToken;

        if c.is_digit(10) {
            let number = self.consume_number();
            kind = TokenKind::Number(number)
        }

        let end = self.cur_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);

        Some(Token::new(kind, span))
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    fn get_path() -> String {
        let args: Vec<String> = env::args().collect();
        args[2].to_owned()
    }

    #[test]
    fn lex_num() -> Result<(), Box<dyn std::error::Error>> {
        let path = get_path();
        let content = &fs::read_to_string(path)?;
        let mut l = Lexer::new(content);
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenKind::Number(12), TextSpan::new(0, 2, "12".to_string()))
        );
        Ok(())
    }
}
