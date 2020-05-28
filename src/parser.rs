use crate::lexer::{ Token, Lexer, TokenKind };
use crate::error::{ QccError, ErrorType };

pub struct Parser {
    pub tokens: Vec<Token>,
    pos: usize,
    pub num: usize,
}

impl Parser {
    pub fn new(s: &str) -> Result<Parser, QccError> {
        let mut lexer = Lexer::new(s);
        let mut tokens = Vec::new();
        let mut next_token = lexer.next_token()?;
        loop {
            match &next_token.kind {
                TokenKind::End => {
                    tokens.push(next_token.clone());
                    break;
                },
                token => tokens.push(next_token.clone()),
            }
            next_token = lexer.next_token()?;
        }
        Ok(Parser {
            tokens,
            pos: 0,
            num: 0,
        })
    }

    pub fn parse(&mut self) -> Result<(), QccError> {
        let a = self.alloc()?;
        self.multiplitive_ops(a)?;
        Ok(())
    }

    fn alloc(&mut self) -> Result<usize, QccError> {
        if let TokenKind::Number(i) = self.tokens[self.pos].kind {
            println!("\t%{} = alloca i32, align 4", self.num);
            println!("\tstore i32 {}, i32* %{}, align 4", i, self.num);
            println!("\t%{} = load i32, i32* %{}, align 4", self.num + 1, self.num);
            self.num += 2;
            self.pos += 1;
            Ok(self.num - 1)
        } else {
            Err(QccError{
                kind: ErrorType::UnexpectedToken,
                pos: self.tokens[self.pos].pos,
                message: String::from("This expression expected number"),
            })
        }
    }

    fn additonal_ops(&mut self, initial_pos: usize) -> Result<usize, QccError> {
        let mut add_pos = initial_pos;
        loop {
            match self.tokens[self.pos].kind {
                TokenKind::Plus => {
                    if let TokenKind::Number(i) = self.tokens[self.pos + 1].kind {
                        println!("\t%{} = add i32 {}, %{}", self.num, i, add_pos);
                    } else {
                        return Err(QccError {
                            kind: ErrorType::UnexpectedToken,
                            pos: self.tokens[self.pos].pos,
                            message: String::from("This expression expected number"),
                        });
                    }
                },
                TokenKind::Minus => {
                    if let TokenKind::Number(i) = self.tokens[self.pos + 1].kind {
                        println!("\t%{} = sub i32 {}, %{}", self.num, i, add_pos);
                    } else {
                        return Err(QccError{
                            kind: ErrorType::UnexpectedToken,
                            pos: self.tokens[self.pos].pos,
                            message: String::from("This expression expected number"),
                        });
                    }
                },
                TokenKind::End => {
                    break;
                },
                _ => return Err(QccError{
                    kind: ErrorType::UnexpectedToken,
                    pos: self.tokens[self.pos].pos,
                    message: String::from("This expression expected + or -"),
                }),
            }
            self.pos += 2;
            self.num += 1;
            add_pos += 1;
        }
        Ok(self.num)
    }

    fn multiplitive_ops(&mut self, initial_pos: usize) -> Result<usize, QccError> {
        self.additonal_ops(initial_pos)
    }
}
