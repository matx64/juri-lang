use crate::ast::lexer::Lexer;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Token, TokenKind};

pub struct Parser<'a> {
    pub symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>,
    pub lexer: Lexer<'a>,
    pub current_token: Option<Token>,
    pub expected_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>, input: Vec<char>) -> Self {
        Self {
            symbol_table: symbol_table.clone(),
            lexer: Lexer::new(input, symbol_table.clone()),
            current_token: None,
            expected_token: None,
        }
    }

    pub fn start(&mut self) {
        while self.lexer.input_pos < self.lexer.input.len() {
            println!("{:?}", self.lexer.next_token());
        }

        // match token.unwrap().kind {
        //     TokenKind::IntegerKeyword
        //     | TokenKind::FloatKeyword
        //     | TokenKind::CharKeyword
        //     | TokenKind::StringKeyword => self.declaration(),
        //     _ => self.command(),
        // }
    }

    pub fn _declaration(&mut self) {}

    pub fn _command(&mut self) {}
}
