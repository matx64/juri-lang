use super::{Token, TokenKind};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Lexer<'a> {
    pub symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>,
    pub input: Vec<char>,
    pub input_pos: usize,
    pub ch: Option<char>,
    pub lex: String,
    pub state: u8,
    pub token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Vec<char>, symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>) -> Self {
        Self {
            symbol_table,
            input,
            input_pos: 0,
            ch: None,
            lex: String::with_capacity(32),
            state: 0,
            token: None,
        }
    }

    fn is_letter(&self) -> bool {
        match self.ch {
            Some(ch) => (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z'),
            None => false,
        }
    }

    fn is_digit(&self) -> bool {
        match self.ch {
            Some(ch) => ch >= '0' && ch <= '9',
            None => false,
        }
    }

    fn next_char(&mut self) {
        if self.input_pos < self.input.len() {
            self.ch = Some(self.input[self.input_pos]);
            self.input_pos += 1;

            if self.ch == Some('\r') {
                self.next_char(); // CRLF
            }
        } else {
            self.ch = None; // EOF
            self.input_pos += 1;
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.state = 0;
        self.lex.clear();
        self.token = None;

        while self.state != 99 {
            self.next_char();

            match self.state {
                0 => self.state_0(),
                1 => self.state_1(),
                2 => self.state_2(),
                3 => self.state_3(),
                4 => self.state_4(),
                5 => self.state_5(),
                6 => self.state_6(),
                7 => self.state_7(),
                8 => self.state_8(),
                9 => self.state_9(),
                10 => self.state_10(),
                11 => self.state_11(),
                12 => self.state_12(),
                13 => self.state_13(),
                _ => unreachable!(),
            }
        }

        self.token.clone()
    }

    fn state_0(&mut self) {
        if self.ch.is_none() {
            self.state = 99;
            return;
        }

        let ch = self.ch.unwrap();

        if ch == ' ' {
            return;
        }

        self.lex.push(ch);

        if self.is_letter() || ch == '_' {
            self.state = 1;
        } else if self.is_digit() {
            self.state = 2;
        } else if ch == '.' {
            self.state = 3;
        } else if ch == '=' || ch == '<' || ch == '>' || ch == '!' {
            self.state = 5;
        } else if ch == '&' || ch == '|' {
            self.state = 6;
        } else if ch == '/' {
            self.state = 7;
        } else if ch == '\'' {
            self.state = 10;
        } else if ch == '\"' {
            self.state = 12;
        } else if ch == ';'
            || ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == '&'
            || ch == '('
            || ch == ')'
            || ch == '{'
            || ch == '}'
            || ch == '['
            || ch == ']'
        {
            self.state = 13;
        } else if ch == '\n' {
            self.lex.clear();
        }
    }

    fn state_1(&mut self) {
        if self.is_letter() || self.is_digit() {
            self.lex.push(self.ch.unwrap());
        } else {
            let kind = self.symbol_table.borrow();
            let kind = kind.get(self.lex.as_str());

            self.token = match kind {
                Some(val) => Some(Token {
                    lex: None,
                    kind: val.clone(),
                }),
                None => Some(Token {
                    lex: Some(self.lex.clone()),
                    kind: TokenKind::Identifier,
                }),
            };

            self.input_pos -= 1;
            self.state = 99;
        }
    }

    fn state_2(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch.unwrap());
        } else if self.ch.is_some() && self.ch.unwrap() == '.' {
            self.lex.push(self.ch.unwrap());
            self.state = 3;
        } else {
            self.token = Some(Token {
                lex: Some(self.lex.clone()),
                kind: TokenKind::IntegerValue,
            });
            self.input_pos -= 1;
            self.state = 99;
        }
    }

    fn state_3(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch.unwrap());
            self.state = 4;
        } else if self.ch.is_none() {
            // eof error
            panic!();
        } else {
            // invalid lex error
            // self.lex.push(self.ch.unwrap());
            panic!();
        }
    }

    fn state_4(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch.unwrap());
        } else {
            self.token = Some(Token {
                lex: Some(self.lex.clone()),
                kind: TokenKind::FloatValue,
            });
            self.input_pos -= 1;
            self.state = 99;
        }
    }

    fn state_5(&mut self) {
        if self.ch.is_some() && self.ch.unwrap() == '=' {
            self.lex.push(self.ch.unwrap());
            self.input_pos += 1;
        }
        self.token = Some(Token {
            lex: None,
            kind: self
                .symbol_table
                .borrow()
                .get(self.lex.as_str())
                .unwrap()
                .clone(),
        });
        self.input_pos -= 1;
        self.state = 99;
    }

    fn state_6(&mut self) {
        self.lex.push(self.ch.unwrap());

        if self.ch.is_some() && self.lex.starts_with(self.ch.unwrap()) {
            self.token = Some(Token {
                lex: None,
                kind: self
                    .symbol_table
                    .borrow()
                    .get(self.lex.as_str())
                    .unwrap()
                    .clone(),
            });
            self.state = 99;
        } else {
            // invalid lex error
            panic!();
        }
    }

    fn state_7(&mut self) {
        if self.ch.is_some() && self.ch.unwrap() == '*' {
            self.lex.clear();
            self.state = 7;
        } else {
            self.state = 99;
        }
    }

    fn state_8(&mut self) {
        if self.ch.is_none() {
            // eof error
            panic!();
        } else if self.ch.unwrap() == '*' {
            self.state = 8;
        }
    }

    fn state_9(&mut self) {
        if self.ch.is_none() {
            // eof error
            panic!();
        } else if self.ch.unwrap() == '/' {
            self.state = 0;
        } else if self.ch.unwrap() != '*' {
            self.state = 7;
        }
    }

    fn state_10(&mut self) {
        if self.ch.is_none() {
            // eof error
            panic!();
        } else if !self.ch.unwrap().is_ascii() {
            // invalid lex error
            // self.lex.push(self.ch);
            panic!();
        } else {
            self.lex.push(self.ch.unwrap());
            self.state = 11;
        }
    }

    fn state_11(&mut self) {
        if self.ch.is_none() {
            // eof error
            panic!();
        } else if self.ch.unwrap() != '\'' {
            // invalid lex error
            // self.lex.push(self.ch.unwrap());
            panic!();
        } else {
            self.lex.push(self.ch.unwrap());
            self.token = Some(Token {
                lex: Some(self.lex.clone()),
                kind: TokenKind::CharValue,
            });
            self.state = 99;
        }
    }

    fn state_12(&mut self) {
        if self.ch.is_none() {
            // eof error
            panic!();
        } else if !self.ch.unwrap().is_ascii() {
            // invalid lex error
            // self.lex.push(self.ch.unwrap());
            panic!();
        } else {
            self.lex.push(self.ch.unwrap());

            if self.ch.unwrap() == '\"' {
                self.token = Some(Token {
                    lex: Some(self.lex.clone()),
                    kind: TokenKind::StringValue,
                });
                self.state = 99;
            }
        }
    }

    fn state_13(&mut self) {
        self.token = Some(Token {
            lex: None,
            kind: self
                .symbol_table
                .borrow()
                .get(self.lex.as_str())
                .unwrap()
                .clone(),
        });
        self.input_pos -= 1;
        self.state = 99;
    }
}
