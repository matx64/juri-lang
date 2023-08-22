use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Token, TokenKind};

pub struct Parser<'a> {
    pub symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>,
    pub current_token: Option<Token>,
    pub expected_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>) -> Self {
        Self {
            symbol_table,
            current_token: None,
            expected_token: None,
        }
    }

    pub fn start(&mut self, token: Option<Token>) {
        println!("{:?}", token);

        if token.is_none() {
            return;
        }

        self.current_token = token.clone();

        match token.unwrap().kind {
            TokenKind::IntegerKeyword
            | TokenKind::FloatKeyword
            | TokenKind::CharKeyword
            | TokenKind::StringKeyword => self.declaration(),
            _ => self.command(),
        }
    }

    pub fn declaration(&mut self) {}

    pub fn command(&mut self) {}
}
