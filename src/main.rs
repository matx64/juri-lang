use std::fs;

use juri_lang::Lexer;

fn main() {
    let file = fs::read_to_string("examples/input.juri").unwrap();

    let mut lexer = Lexer::new();

    let lexemes = lexer.get_lexemes(file.chars());

    println!("{:?}", lexemes);
}
