use super::{create_symbol_table, Token, TokenKind};
use std::collections::HashMap;

pub struct Lexer<'a> {
    pub symbol_table: HashMap<&'a str, TokenKind>,
    pub input: Vec<char>,
    pub input_pos: usize,
    pub ch: char,
    pub lex: String,
    pub state: u8,
    pub token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            symbol_table: create_symbol_table(),
            input,
            input_pos: 0,
            ch: '#',
            lex: String::with_capacity(32),
            state: 0,
            token: None,
        }
    }

    pub fn is_letter(&self) -> bool {
        (self.ch >= 'a' && self.ch <= 'z') || (self.ch >= 'A' && self.ch <= 'Z')
    }

    pub fn is_digit(&self) -> bool {
        self.ch >= '0' && self.ch <= '9'
    }

    fn next_char(&mut self) {
        if self.input_pos < self.input.len() {
            self.ch = self.input[self.input_pos];
            self.input_pos += 1;

            if self.ch == '\r' {
                self.next_char(); // CRLF
            }
        } else {
            self.ch = '#'; // EOF
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
                14 => self.state_14(),
                _ => unreachable!(),
            }
        }

        self.token.clone()
    }

    pub fn state_0(&mut self) {
        if self.ch == ' ' {
            return;
        }

        self.lex.push(self.ch);

        if self.is_letter() || self.ch == '_' {
            self.state = 1;
        } else if self.is_digit() {
            self.state = 2;
        } else if self.ch == '.' {
            self.state = 3;
        } else if self.ch == '<' || self.ch == '>' || self.ch == '!' {
            self.state = 5;
        } else if self.ch == '&' || self.ch == '|' {
            self.state = 6;
        } else if self.ch == '/' {
            self.state = 7;
        } else if self.ch == '\'' {
            self.state = 10;
        } else if self.ch == '\"' {
            self.state = 12;
        } else if self.ch == '='
            || self.ch == '+'
            || self.ch == '-'
            || self.ch == '*'
            || self.ch == ','
            || self.ch == '('
            || self.ch == ')'
            || self.ch == '{'
            || self.ch == '}'
            || self.ch == '['
            || self.ch == ']'
            || self.ch == ';'
        {
            self.state = 14;
        } else if self.ch == '\n' {
            self.lex.clear();
        } else if self.ch == '#' {
            self.state = 99;
        }
    }

    pub fn state_1(&mut self) {
        if self.is_letter() || self.is_digit() {
            self.lex.push(self.ch);
        } else {
            let kind = self.symbol_table.get(self.lex.as_str());

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

    pub fn state_2(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch);
        } else if self.ch == '.' {
            self.lex.push(self.ch);
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

    pub fn state_3(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch);
            self.state = 4;
        } else if self.ch == '#' {
            // eof error
            panic!();
        } else {
            // invalid lex error
            // self.lex.push(self.ch);
            panic!();
        }
    }

    pub fn state_4(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch);
        } else {
            self.token = Some(Token {
                lex: Some(self.lex.clone()),
                kind: TokenKind::FloatValue,
            });
            self.input_pos -= 1;
            self.state = 99;
        }
    }

    pub fn state_5(&mut self) {
        if self.ch == '=' {
            self.lex.push(self.ch);
        } else {
            self.token = Some(Token {
                lex: None,
                kind: self.symbol_table.get(self.lex.as_str()).unwrap().clone(),
            });
            self.input_pos -= 1;
        }
        self.state = 99;
    }

    pub fn state_6(&mut self) {
        self.lex.push(self.ch);

        if self.lex.starts_with(self.ch) {
            self.token = Some(Token {
                lex: None,
                kind: self.symbol_table.get(self.lex.as_str()).unwrap().clone(),
            });
            self.state = 99;
        } else {
            // invalid lex error
            panic!();
        }
    }

    pub fn state_7(&mut self) {
        if self.ch == '*' {
            self.lex.clear();
            self.state = 7;
        } else {
            self.state = 99;
        }
    }

    pub fn state_8(&mut self) {
        if self.ch == '#' {
            // eof error
            panic!();
        } else if self.ch == '*' {
            self.state = 8;
        }
    }

    pub fn state_9(&mut self) {
        if self.ch == '#' {
            // eof error
            panic!();
        } else if self.ch == '/' {
            self.state = 0;
        } else if self.ch != '*' {
            self.state = 7;
        }
    }

    pub fn state_10(&mut self) {
        if self.ch == '#' {
            // eof error
            panic!();
        } else if !self.ch.is_ascii() {
            // invalid lex error
            // self.lex.push(self.ch);
            panic!();
        } else {
            self.lex.push(self.ch);
            self.state = 11;
        }
    }

    pub fn state_11(&mut self) {
        if self.ch == '#' {
            // eof error
            panic!();
        } else if self.ch != '\'' {
            // invalid lex error
            // self.lex.push(self.ch);
            panic!();
        } else {
            self.token = Some(Token {
                lex: Some(self.lex.clone()),
                kind: TokenKind::CharValue,
            });
            self.lex.push(self.ch);
            self.state = 99;
        }
    }

    pub fn state_14(&mut self) {
        self.token = Some(Token {
            lex: None,
            kind: self.symbol_table.get(self.lex.as_str()).unwrap().clone(),
        });
        self.state = 99;
    }
}
