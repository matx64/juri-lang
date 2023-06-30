use std::fs;

struct Lexer {
    pub lex: String,
    pub state: u8,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            lex: String::new(),
            state: 0,
        }
    }

    pub fn is_letter(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
    }

    pub fn state_0(&mut self, ch: char) {
        if ch == ' ' {
            return;
        }

        self.lex.push(ch);

        if self.is_letter(ch) {
            self.state = 1;
        }
    }

    pub fn state_1(&mut self, ch: char) {
        if self.is_letter(ch) || ch.is_numeric() {
            self.lex.push(ch);
        } else {
            self.state = 99;
        }
    }

    pub fn get_lexemes(&mut self, file: String) -> Vec<String> {
        let mut lexemes = vec![];

        self.lex.clear();

        for ch in file.chars() {
            match self.state {
                0 => self.state_0(ch),
                1 => self.state_1(ch),
                99 => {
                    lexemes.push(self.lex.clone());
                    self.lex.clear();
                    self.state = 0;
                }
                _ => {}
            }
        }

        lexemes
    }
}

fn main() {
    let file = fs::read_to_string("input.juri").unwrap();

    let mut lexer = Lexer::new();

    let mut _lexemes = lexer.get_lexemes(file);
}
