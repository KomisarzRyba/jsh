use std::{char, os, str::Chars};

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

    fn peek(&self) -> Option<char> {
        if self.cur_pos >= self.input.len() {
            return None;
        }
        let c = self.cur_char();
        Some(c)
    }

    fn next_pos(&mut self) {
        self.cur_pos += 1;
    }

    fn consume_number(&mut self) -> i64 {
        let mut num = 0i64;
        while let Some(c) = self.peek() {
            let c = c.to_digit(10);
            match c {
                Some(d) => {
                    num = num * 10 + d as i64;
                    self.next_pos();
                }
                None => {
                    break;
                }
            }
        }
        num
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.next_pos();
            } else {
                break;
            }
        }
    }

    fn consume_punctuator(&mut self) -> TokenKind {
        let c = self.cur_char();
        self.next_pos();
        match c {
            '+' => TokenKind::Plus,
            // add more
            _ => TokenKind::BadToken,
        }
    }

    fn consume(&mut self) -> Option<Token> {
        let start = self.cur_pos;
        let mut kind = TokenKind::BadToken;
        let c = self.cur_char();
        if c.is_digit(10) {
            kind = TokenKind::Number(self.consume_number());
        } else if c.is_ascii_whitespace() {
            self.consume_whitespace();
            kind = TokenKind::Whitespace;
        } else if c.is_ascii_punctuation() {
            kind = self.consume_punctuator();
        } else {
            self.next_pos();
        }

        let end = self.cur_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);

        Some(Token::new(kind, span))
    }

    fn next(&mut self) -> Option<Token> {
        if self.cur_pos > self.input.len() {
            return None;
        }
        if self.cur_pos == self.input.len() {
            let t = Token::new(
                TokenKind::EOF,
                TextSpan::new(self.cur_pos, self.cur_pos, '\0'.to_string()),
            );
            self.next_pos();
            return Some(t);
        }

        self.consume()
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
        // let mut tokens: Vec<Token> = Vec::new();
        // while let Some(t) = l.next() {
        //     tokens.push(t);
        // }
        // println!("{:?}", tokens);
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenKind::Number(12), TextSpan::new(0, 2, "12".to_string()))
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenKind::Whitespace, TextSpan::new(2, 3, ' '.to_string()))
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenKind::Plus, TextSpan::new(3, 4, "+".to_string()))
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenKind::Whitespace, TextSpan::new(4, 5, '\n'.to_string()))
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenKind::EOF, TextSpan::new(5, 5, '\0'.to_string()))
        );
        Ok(())
    }
}
