use std::str::Chars;

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

        if self.is_letter(ch) {
            self.state = 1;
        } else if ch.is_numeric() {
            self.state = 2;
        } else if ch == '=' || ch == '+' {
            self.state = 99;
        } else if ch == ';' {
            self.state = 99;
        } else if ch == '\n' {
            self.lex.clear();
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
        } else {
            self.keep_char = true;
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

            match self.keep_char {
                false => {
                    ch_opt = chars.next();
                }
                true => {
                    ch_opt = Some(ch_val);
                }
            }
        }

        if self.lex.len() > 0 {
            lexemes.push(self.lex.clone());
        }

        lexemes
    }
}
