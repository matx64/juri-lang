use std::fs;

struct Symbol<'a> {
    lex: &'a str,
    addr: i32,
    t: SymbolType,
}

enum SymbolType {
    Int,
}

struct Lexer {
    pub lex: String,
    pub state: u8,
}

impl Lexer {
    pub fn new(&self) -> Self {
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

    pub fn state_1(&mut self, ch: char) {}
}

fn main() {
    let file = fs::read_to_string("input.juri").unwrap();

    for ch in file.chars() {
        if ch == '\n' {
            continue;
        };
        println!("{}", ch);
    }
}
