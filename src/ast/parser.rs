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

    pub fn _start(&mut self) {}
}
