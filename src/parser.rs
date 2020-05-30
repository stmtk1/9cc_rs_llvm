use crate::lexer::{ Token, Lexer, TokenKind };
use crate::error::{ QccError, ErrorType };
use crate::token_tree::{ TokenTree };

pub struct Parser {
    pub tokens: TokenTree,
    pub num: usize,
}

impl Parser {
    pub fn new(s: &str) -> Result<Parser, QccError> {
        Ok(Parser {
            tokens: TokenTree::new(s)?,
            num: 0,
        })
    }

    pub fn parse(&mut self) -> Result<usize, QccError> {
        Parser::parse_tokens(0, &self.tokens)
    }

    fn parse_tokens(n: usize, tokens: &TokenTree) -> Result<usize, QccError> {
        match tokens.kind {
            TokenKind::Number(i) => {
                println!("\t%{} = alloca i32, align 4", n);
                println!("\tstore i32 {}, i32* %{}, align 4", i, n);
                println!("\t%{} = load i32, i32* %{}, align 4", n + 1, n);
                Ok(n + 1)
            },
            TokenKind::Plus => {
                let a = Parser::parse_tokens(n, &tokens.lhs.as_ref().unwrap().borrow())?;
                let b = Parser::parse_tokens(a + 1, &tokens.rhs.as_ref().unwrap().borrow())?;
                println!("\t%{} = add i32 %{}, %{}", b + 1, a, b);
                Ok(b + 1)
            },
            TokenKind::Minus => {
                let a = Parser::parse_tokens(n, &tokens.lhs.as_ref().unwrap().borrow())?;
                let b = Parser::parse_tokens(a + 1, &tokens.rhs.as_ref().unwrap().borrow())?;
                println!("\t%{} = sub i32 %{}, %{}", b + 1, a, b);
                Ok(b + 1)
            },
            TokenKind::Multiply => {
                let a = Parser::parse_tokens(n, &tokens.lhs.as_ref().unwrap().borrow())?;
                let b = Parser::parse_tokens(a + 1, &tokens.rhs.as_ref().unwrap().borrow())?;
                println!("\t%{} = mul i32 %{}, %{}", b + 1, a, b);
                Ok(b + 1)
            },
            TokenKind::Devide => {
                let a = Parser::parse_tokens(n, &tokens.lhs.as_ref().unwrap().borrow())?;
                let b = Parser::parse_tokens(a + 1, &tokens.rhs.as_ref().unwrap().borrow())?;
                println!("\t%{} = sdiv i32 %{}, %{}", b + 1, a, b);
                Ok(b + 1)
            },
            _ => {
                Err(QccError{
                    kind: ErrorType::UnexpectedToken,
                    pos: tokens.pos,
                    message: String::from("unexpected token"),
                })
            }
        }
    }
}
