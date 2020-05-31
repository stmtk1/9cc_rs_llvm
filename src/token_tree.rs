use crate::lexer::{ TokenKind, Token, Lexer };
use crate::error::{ QccError, ErrorType };
use std::rc::Rc;
use std::cell::RefCell;

pub struct TokenTree {
    pub kind: TreeKind,
    pub pos: usize,
    pub lhs: Option<Rc<RefCell<TokenTree>>>,
    pub rhs: Option<Rc<RefCell<TokenTree>>>,
}

pub enum TreeKind {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Devide,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
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

    fn expr(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        TokenTree::equal(n, list)
    }

    fn equal(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let mut token = TokenTree::relational(n, list)?;

        loop {
            let m = *n;
            let now = list[m].clone();
            match now.kind {
                TokenKind::Equal => {
                    match list[*n + 1].kind {
                        TokenKind::Equal => {
                            let m = *n;
                            *n += 2;
                            token = TokenTree {
                                kind: TreeKind::Equal,
                                pos: list[m].pos,
                                lhs: Some(Rc::new(RefCell::new(token))),
                                rhs: Some(Rc::new(RefCell::new(TokenTree::relational(n, list)?))),
                            };
                        },
                        TokenKind::Not => {
                            let m = *n;
                            *n += 2;
                            token = TokenTree {
                                kind: TreeKind::NotEqual,
                                pos: list[m].pos,
                                lhs: Some(Rc::new(RefCell::new(token))),
                                rhs: Some(Rc::new(RefCell::new(TokenTree::relational(n, list)?))),
                            };
                        },
                        _ => return Err(QccError {
                            kind: ErrorType::UnexpectedToken,
                            pos: list[*n].pos,
                            message: String::from("unexpected token"),
                        }),
                    }
                },
                _ => return Ok(token),
            }
        }
    }

    fn relational(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let mut token = TokenTree::add(n, list)?;

        loop {
            let m = *n;
            let now = list[m].clone();
            match now.kind {
                TokenKind::Less | TokenKind::Greater => {
                    match list[*n + 1].kind {
                        TokenKind::Equal => {
                            let m = *n;
                            *n += 2;
                            let kind = match now.kind {
                                TokenKind::Less => TreeKind::LessEqual,
                                _ => TreeKind::GreaterEqual,
                            };
                            token = TokenTree {
                                kind,
                                pos: list[m].pos,
                                lhs: Some(Rc::new(RefCell::new(token))),
                                rhs: Some(Rc::new(RefCell::new(TokenTree::add(n, list)?))),
                            };
                        },
                        _ => {
                            let m = *n;
                            *n += 1;
                            token = TokenTree {
                                kind: kind_convert(&list[m])?,
                                pos: list[m].pos,
                                lhs: Some(Rc::new(RefCell::new(token))),
                                rhs: Some(Rc::new(RefCell::new(TokenTree::add(n, list)?))),
                            }
                        },
                    }
                },
                _ => return Ok(token),
            }
        }
    }

    fn primary(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let m = *n;
        match list[m].kind {
            TokenKind::Number(_) => {
                *n += 1;
                Ok(TokenTree {
                    kind: kind_convert(&list[m])?,
                    pos: list[m].pos,
                    lhs: None,
                    rhs: None,
                })
            },
            TokenKind::LParen => {
                *n += 1;
                let ret = TokenTree::expr(n, list)?;
                match list[*n].kind {
                    TokenKind::RParen => {
                        *n += 1;
                        Ok(ret)
                    },
                    _ => Err(QccError{
                        kind: ErrorType::UnexpectedToken,
                        pos: list[*n].pos,
                        message: String::from("this token expected )"),
                    })
                }
            },
            _ => {
                Err(QccError {
                    kind: ErrorType::UnexpectedToken,
                    pos: list[*n].pos,
                    message: String::from("this expression expected a number"),
                })
            }
        }
    }

    fn unary(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        match list[*n].kind {
            TokenKind::Plus => {
                *n += 1;
                TokenTree::primary(n, list)
            },
            TokenKind::Minus => {
                let m = *n;
                *n += 1;
                let zero = TokenTree {
                    kind: TreeKind::Number(0),
                    pos: list[m].pos,
                    lhs: None,
                    rhs: None,
                };
                Ok(TokenTree {
                    kind: kind_convert(&list[m])?,
                    pos: list[m].pos,
                    lhs: Some(Rc::new(RefCell::new(zero))),
                    rhs: Some(Rc::new(RefCell::new(TokenTree::primary(n, list)?))),
                })
            },
            _ => TokenTree::primary(n, list),
        }
    }

    fn mul(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let mut token = TokenTree::unary(n, list)?;

        loop {
            let m = *n;
            let now = list[m].clone();
            match now.kind {
                TokenKind::Multiply => {
                    *n += 1;
                    token = TokenTree {
                        kind: kind_convert(&now)?,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::unary(n, list)?))),
                    };
                },
                TokenKind::Devide => {
                    *n += 1;
                    token = TokenTree {
                        kind: kind_convert(&now)?,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::unary(n, list)?))),
                    };
                },
                _ => return Ok(token),
            }
        }
    }

    fn add(n: &mut usize, list: &Vec<Token>) -> Result<TokenTree, QccError> {
        let mut token = TokenTree::mul(n, list)?;

        loop {
            let m = *n;
            let now = list[m].clone();
            match now.kind {
                TokenKind::Plus => {
                    *n += 1;
                    token = TokenTree {
                        kind: kind_convert(&now)?,
                        pos: now.pos,
                        lhs: Some(Rc::new(RefCell::new(token))),
                        rhs: Some(Rc::new(RefCell::new(TokenTree::mul(n, list)?))),
                    };
                },
                TokenKind::Minus => {
                    *n += 1;
                    token = TokenTree {
                        kind: kind_convert(&now)?,
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

fn kind_convert(token: &Token) -> Result<TreeKind, QccError> {
    match token.kind {
        TokenKind::Number(i) => Ok(TreeKind::Number(i)),
        TokenKind::Plus => Ok(TreeKind::Plus),
        TokenKind::Minus => Ok(TreeKind::Minus),
        TokenKind::Multiply => Ok(TreeKind::Multiply),
        TokenKind::Devide => Ok(TreeKind::Devide),
        TokenKind::Less => Ok(TreeKind::Less),
        TokenKind::Greater => Ok(TreeKind::Greater),
        _ => Err(QccError{
            kind: ErrorType::UnexpectedToken,
            pos: token.pos,
            message: String::from("unexpected token"),
        })
    }
}
