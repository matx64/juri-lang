use std::str::Chars;

const EOF: char = '#';

pub struct Lexer {
    pub lex: String,
    pub state: u8,
    pub keep_char: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            lex: String::new(),
            state: 0,
            keep_char: false,
        }
    }

    pub fn is_letter(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
    }

    pub fn state_0(&mut self, ch: char) {
        self.keep_char = false;

        if ch == ' ' {
            return;
        }

        self.lex.push(ch);

        if self.is_letter(ch) || ch == '_' {
            self.state = 1;
        } else if ch.is_numeric() {
            self.state = 2;
        } else if ch == '.' {
            self.state = 3;
        } else if ch == '<' || ch == '>' || ch == '!' {
            self.state = 5;
        } else if ch == '&' || ch == '|' {
            self.state = 6;
        } else if ch == '/' {
            self.state = 7;
        } else if ch == '\'' {
            self.state = 10;
        } else if ch == '\"' {
            self.state = 12;
        } else if ch == '='
            || ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == ','
            || ch == '('
            || ch == ')'
            || ch == '{'
            || ch == '}'
            || ch == '['
            || ch == ']'
        {
            self.state = 99;
        } else if ch == ';' {
            self.state = 99;
        } else if ch == '\n' {
            self.lex.clear();
        } else if ch == EOF {
            self.state = 99;
        }
    }

    pub fn state_1(&mut self, ch: char) {
        if self.is_letter(ch) || ch.is_numeric() {
            self.lex.push(ch);
        } else {
            self.keep_char = true;
            self.state = 99;
        }
    }

    pub fn state_2(&mut self, ch: char) {
        if ch.is_numeric() {
            self.lex.push(ch);
        } else if ch == '.' {
            self.lex.push(ch);
            self.state = 3;
        } else {
            self.keep_char = true;
            self.state = 99;
        }
    }

    pub fn state_3(&mut self, ch: char) {
        if ch.is_numeric() {
            self.lex.push(ch);
            self.state = 4;
        } else if ch == EOF {
            // eof error
        } else {
            self.lex.push(ch);
            // invalid lex error
        }
    }

    pub fn state_4(&mut self, ch: char) {
        if ch.is_numeric() {
            self.lex.push(ch);
        } else {
            self.keep_char = true;
            self.state = 99;
        }
    }

    pub fn state_5(&mut self, ch: char) {
        if ch == '=' {
            self.lex.push(ch);
        } else {
            self.keep_char = true;
        }

        self.state = 99;
    }

    pub fn state_6(&mut self, ch: char) {
        self.lex.push(ch);

        if self.lex.starts_with(ch) {
            self.state = 99;
        } else {
            self.lex.push(ch);
            // invalid lex error
        }
    }

    pub fn state_7(&mut self, ch: char) {
        if ch == '*' {
            self.lex.clear();
            self.state = 7;
        } else {
            self.state = 99;
        }
    }

    pub fn state_8(&mut self, ch: char) {
        if ch == EOF {
            // eof error
        } else if ch == '*' {
            self.state = 8;
        }
    }

    pub fn state_9(&mut self, ch: char) {
        if ch == EOF {
            // eof error
        } else if ch == '/' {
            self.state = 0;
        } else if ch != '*' {
            self.state = 7;
        }
    }

    pub fn state_10(&mut self, ch: char) {
        if ch == EOF {
            // eof error
        } else if !ch.is_ascii() {
            self.lex.push(ch);
            // invalid lex error
        } else {
            self.lex.push(ch);
            self.state = 11;
        }
    }

    pub fn state_11(&mut self, ch: char) {
        if ch == EOF {
            // eof error
        } else if ch != '\'' {
            self.lex.push(ch);
            // invalid lex error
        } else {
            self.lex.push(ch);
            self.state = 99;
        }
    }

    pub fn get_lexemes(&mut self, mut chars: Chars) -> Vec<String> {
        let mut lexemes = vec![];

        let mut ch_opt = chars.next();

        while ch_opt.is_some() {
            let ch_val = ch_opt.unwrap();

            match self.state {
                0 => self.state_0(ch_val),
                1 => self.state_1(ch_val),
                2 => self.state_2(ch_val),
                3 => self.state_3(ch_val),
                99 => {
                    lexemes.push(self.lex.clone());
                    self.lex.clear();
                    self.state = 0;
                }
                _ => {}
            }

            ch_opt = if !self.keep_char {
                chars.next()
            } else {
                Some(ch_val)
            }
        }

        if self.lex.len() > 0 {
            lexemes.push(self.lex.clone());
        }

        lexemes
    }
}
