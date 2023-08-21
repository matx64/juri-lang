mod ast;

use ast::Ast;
use std::fs;

fn main() {
    let input: Vec<char> = fs::read_to_string("examples/input.juri")
        .unwrap()
        .chars()
        .collect();

    let mut ast_ = Ast::new();

    ast_.run(input);
}
