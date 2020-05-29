use crate::lexer::{ TokenKind, Token, Lexer };
use crate::error::{ QccError, ErrorType };
use std::rc::Rc;
use std::cell::RefCell;

pub struct TokenTree {
    pub kind: TokenKind,
    pub pos: usize,
    pub lhs: Option<Rc<RefCell<TokenTree>>>,
    pub rhs: Option<Rc<RefCell<TokenTree>>>,
}

impl TokenTree {
    pub fn new(s: &str) -> Result<TokenTree, QccError> {
        let list = get_token_list(s)?;
        let mut n = 0;
        let ret = TokenTree::expr(&mut n, &list)?;
        match list[n].kind {
            TokenKind::End => Ok(ret),
            _ => {
                Err(QccError {
                    kind: ErrorType::UnexpectedToken,
                    pos: list[n].pos,
                    message: String::from("unexpected EOF"),
                })
            }
        }
    }

    fn primary(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let m = *n;
        if let TokenKind::Number(_) = list[m].kind {
            *n += 1;
            Ok(TokenTree {
                kind: list[m].kind.clone(),
                pos: list[m].pos,
                lhs: None,
                rhs: None,
            })
        } else {
            Err(QccError {
                kind: ErrorType::UnexpectedToken,
                pos: list[*n].pos,
                message: String::from("this expression expected a number"),
            })
        }
    }

    fn mul(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let mut token = TokenTree::primary(n, list)?;

        loop {
            let m = *n;
            let now = list[m].clone();
            match now.kind {
                TokenKind::Multiply => {
                    *n += 1;
                    token = TokenTree {
                        kind: now.kind,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::primary(n, list)?))),
                    };
                },
                TokenKind::Devide => {
                    *n += 1;
                    token = TokenTree {
                        kind: now.kind,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::primary(n, list)?))),
                    };
                },
                _ => return Ok(token),
            }
        }
    }

    fn expr(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let mut token = TokenTree::mul(n, list)?;

        loop {
            let m = *n;
            let now = list[m].clone();
            match now.kind {
                TokenKind::Plus => {
                    *n += 1;
                    token = TokenTree {
                        kind: now.kind,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::mul(n, list)?))),
                    };
                },
                TokenKind::Minus => {
                    *n += 1;
                    token = TokenTree {
                        kind: now.kind,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::mul(n, list)?))),
                    };
                },
                _ => return Ok(token),
            }
        }
    }
}

fn get_token_list(s: &str) -> Result<Vec<Token>, QccError> {

    let mut lexer = Lexer::new(s);
    let mut next_token = lexer.next_token()?;
    let mut tokens = Vec::new();

    loop {
        tokens.push(next_token.clone());
        if TokenKind::End == next_token.kind {
            break;
        }
        next_token = lexer.next_token()?;
    }
    Ok(tokens)
}
