use crate::ast::lexer::Lexer;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Token, TokenKind};

pub struct Parser<'a> {
    pub symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>,
    pub lexer: Lexer<'a>,
    pub current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>, input: Vec<char>) -> Self {
        let mut lexer = Lexer::new(input, symbol_table.clone());
        let first_token = lexer.next_token();
        Self {
            symbol_table: symbol_table.clone(),
            lexer,
            current_token: first_token,
        }
    }

    pub fn start(&mut self) {
        match &self.current_token.kind {
            TokenKind::IntegerKeyword
            | TokenKind::FloatKeyword
            | TokenKind::CharKeyword
            | TokenKind::StringKeyword => self.declaration(),
            _ => self.command(),
        }
    }

    fn match_token(&mut self, expected_kind: TokenKind) -> bool {
        self.current_token = self.lexer.next_token();

        self.current_token.kind == expected_kind
    }

    fn declaration(&mut self) {
        let declaration_type = self.current_token.kind.clone();

        if self.match_token(TokenKind::Identifier) {
            if self.match_token(TokenKind::Atrib) {
                let expected_kind = match declaration_type {
                    TokenKind::IntegerKeyword => TokenKind::IntegerValue,
                    TokenKind::FloatKeyword => TokenKind::FloatValue,
                    TokenKind::CharKeyword => TokenKind::CharValue,
                    TokenKind::StringKeyword => TokenKind::StringValue,
                    _ => unreachable!(),
                };

                if self.match_token(expected_kind) {
                    if self.match_token(TokenKind::Semicolon) {}
                }
            }
        }
    }

    fn command(&mut self) {}
}
