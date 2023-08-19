mod ast;

use ast::lexer::Lexer;
use std::fs;

fn main() {
    let input = fs::read_to_string("examples/input.juri")
        .unwrap()
        .chars()
        .collect();

    let mut lexer = Lexer::new(input);

    while lexer.input_pos < lexer.input.len() {
        println!("{:?}", lexer.next_token());
    }
}
