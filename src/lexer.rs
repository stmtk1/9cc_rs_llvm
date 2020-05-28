use crate::error::{ QccError, ErrorType };

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    Number(i32),
    Plus,
    Minus,
    End,
}

pub struct Lexer <'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            input: &s.as_bytes(),
            pos: 0
        }
    }

    pub fn next_token(&mut self) -> Result<Token, QccError> {
        while self.pos < self.input.len() && self.input[self.pos] == b' ' {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return Ok(Token{
                kind: TokenKind::End,
                pos: self.pos,
            });
        }
        match self.input[self.pos] {
            b'0'..=b'9' => self.lex_number(),
            b'+' => self.lex_plus(),
            b'-' => self.lex_minus(),
            _ => Err(QccError{
                kind: ErrorType::UnexpectedChar,
                pos: self.pos,
                message: String::from("unknown expression"),
            })
        }
    }

    fn lex_plus(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token{
            kind: TokenKind::Plus,
            pos: self.pos - 1,
        })
    }

    fn lex_minus(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token{
            kind: TokenKind::Minus,
            pos: self.pos -1,
        })
    }

    fn lex_number(&mut self) -> Result<Token, QccError> {
        let start = self.pos;
        let end = self.recognize_multiple_chars(|b| b"0123456789".contains(&b));
        self.pos = end;
        let num = std::str::from_utf8(&self.input[start..end]).unwrap().parse::<i32>().unwrap();
        Ok(Token{
            kind: TokenKind::Number(num),
            pos: start,
        })
    }

    fn recognize_multiple_chars(&self, mut f: impl FnMut(u8) -> bool) -> usize {
        let mut pos = self.pos;

        while pos < self.input.len() && f(self.input[pos]) {
            pos += 1;
        }
        pos
    }
}

