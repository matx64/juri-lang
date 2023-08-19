const EOF: char = '#';

pub struct Lexer {
    pub input: Vec<char>,
    pub input_pos: usize,
    pub ch: char,
    pub lex: String,
    pub state: u8,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            input_pos: 0,
            ch: '#',
            lex: String::new(),
            state: 0,
        }
    }

    pub fn is_letter(&self) -> bool {
        (self.ch >= 'a' && self.ch <= 'z') || (self.ch >= 'A' && self.ch <= 'Z')
    }

    pub fn is_digit(&self) -> bool {
        self.ch >= '0' && self.ch <= '9'
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
        {
            self.state = 99;
        } else if self.ch == ';' {
            self.state = 99;
        } else if self.ch == '\n' {
            self.lex.clear();
        } else if self.ch == EOF {
            self.state = 99;
        }
    }

    pub fn state_1(&mut self) {
        if self.is_letter() || self.is_digit() {
            self.lex.push(self.ch);
        } else {
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
            self.input_pos -= 1;
            self.state = 99;
        }
    }

    pub fn state_3(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch);
            self.state = 4;
        } else if self.ch == EOF {
            // eof error
        } else {
            self.lex.push(self.ch);
            // invalid lex error
        }
    }

    pub fn state_4(&mut self) {
        if self.is_digit() {
            self.lex.push(self.ch);
        } else {
            self.input_pos -= 1;
            self.state = 99;
        }
    }

    pub fn state_5(&mut self) {
        if self.ch == '=' {
            self.lex.push(self.ch);
        } else {
            self.input_pos -= 1;
        }
        self.state = 99;
    }

    pub fn state_6(&mut self) {
        self.lex.push(self.ch);

        if self.lex.starts_with(self.ch) {
            self.state = 99;
        } else {
            self.lex.push(self.ch);
            // invalid lex error
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
        if self.ch == EOF {
            // eof error
        } else if self.ch == '*' {
            self.state = 8;
        }
    }

    pub fn state_9(&mut self) {
        if self.ch == EOF {
            // eof error
        } else if self.ch == '/' {
            self.state = 0;
        } else if self.ch != '*' {
            self.state = 7;
        }
    }

    pub fn state_10(&mut self) {
        if self.ch == EOF {
            // eof error
        } else if !self.ch.is_ascii() {
            self.lex.push(self.ch);
            // invalid lex error
        } else {
            self.lex.push(self.ch);
            self.state = 11;
        }
    }

    pub fn state_11(&mut self) {
        if self.ch == EOF {
            // eof error
        } else if self.ch != '\'' {
            self.lex.push(self.ch);
            // invalid lex error
        } else {
            self.lex.push(self.ch);
            self.state = 99;
        }
    }

    fn next_char(&mut self) {
        if self.input_pos < self.input.len() {
            self.ch = self.input[self.input_pos];
            self.input_pos += 1;

            if self.ch == '\r' {
                self.next_char();
            }
        } else {
            self.ch = '#';
        }
    }

    pub fn next_lexeme(&mut self) -> String {
        self.state = 0;
        self.lex.clear();

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
                _ => unreachable!(),
            }
        }

        self.lex.clone()
    }
}
