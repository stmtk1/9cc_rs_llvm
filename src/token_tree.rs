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
        if let TokenKind::Number(_) = list[0].kind {
            Ok(TokenTree {
                kind: list[0].kind.clone(),
                pos: list[0].pos,
                lhs: None,
                rhs: None,
            })
        } else {
            Err(QccError {
                kind: ErrorType::UnexpectedToken,
                pos: list[0].pos,
                message: String::from("this expression expected a number"),
            })
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
