use crate::error::{ QccError, ErrorType };

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Devide,
    LParen,
    RParen,
    Equal,
    Not,
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
            b'*' => self.lex_multply(),
            b'/' => self.lex_devide(),
            b'(' => self.lex_lparen(),
            b')' => self.lex_rparen(),
            b'=' => self.lex_equal(),
            b'!' => self.lex_not(),
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

    fn lex_multply(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token{
            kind: TokenKind::Multiply,
            pos: self.pos -1,
        })
    }

    fn lex_devide(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token {
            kind: TokenKind::Devide,
            pos: self.pos -1,
        })
    }

    fn lex_lparen(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token {
            kind: TokenKind::LParen,
            pos: self.pos -1,
        })
    }

    fn lex_rparen(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token {
            kind: TokenKind::RParen,
            pos: self.pos -1,
        })
    }

    fn lex_equal(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token {
            kind: TokenKind::Equal,
            pos: self.pos -1,
        })
    }

    fn lex_not(&mut self) -> Result<Token, QccError> {
        self.pos += 1;
        Ok(Token {
            kind: TokenKind::Not,
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

