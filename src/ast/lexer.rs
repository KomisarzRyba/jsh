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
        let c = self.cur_char();
        self.cur_pos += 1;
        if self.cur_pos >= self.input.len() {
            return None;
        }
        Some(c)
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
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

        if Self::is_number_start(&c) {
            let number: i64 = Self::tokenize_number(&c);
            kind = TokenKind::Number(number)
        }

        let end = self.cur_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);

        Some(Token::new(kind, span))
    }
}
