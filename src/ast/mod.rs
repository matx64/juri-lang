pub mod lexer;
pub mod parser;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::parser::Parser;

pub struct Ast<'a> {
    pub symbol_table: Rc<RefCell<HashMap<&'a str, TokenKind>>>,
}

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            symbol_table: Rc::new(RefCell::new(create_symbol_table())),
        }
    }

    pub fn run(&mut self, input: Vec<char>) {
        let mut parser = Parser::new(self.symbol_table.clone(), input);

        parser.start();
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lex: Option<String>,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(lex: Option<String>, kind: TokenKind) -> Self {
        Self { lex, kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    IntegerKeyword,
    IntegerValue,
    FloatKeyword,
    FloatValue,
    BoolKeyword,
    CharKeyword,
    CharValue,
    StringKeyword,
    StringValue,
    True,
    False,
    For,
    While,
    If,
    Else,
    And,
    Or,
    Not,
    Atrib,
    Equals,
    Different,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Semicolon,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    EOF,
}

pub fn create_symbol_table<'a>() -> HashMap<&'a str, TokenKind> {
    let mut symbol_table = HashMap::new();

    symbol_table.insert("int", TokenKind::IntegerKeyword);
    symbol_table.insert("float", TokenKind::FloatKeyword);
    symbol_table.insert("bool", TokenKind::BoolKeyword);
    symbol_table.insert("char", TokenKind::CharKeyword);
    symbol_table.insert("string", TokenKind::StringKeyword);
    symbol_table.insert("true", TokenKind::True);
    symbol_table.insert("false", TokenKind::False);
    symbol_table.insert("for", TokenKind::For);
    symbol_table.insert("while", TokenKind::While);
    symbol_table.insert("if", TokenKind::If);
    symbol_table.insert("else", TokenKind::Else);
    symbol_table.insert("&&", TokenKind::And);
    symbol_table.insert("||", TokenKind::Or);
    symbol_table.insert("!", TokenKind::Not);
    symbol_table.insert("=", TokenKind::Atrib);
    symbol_table.insert("==", TokenKind::Equals);
    symbol_table.insert("!=", TokenKind::Different);
    symbol_table.insert("<", TokenKind::Less);
    symbol_table.insert("<=", TokenKind::LessEquals);
    symbol_table.insert(">", TokenKind::Greater);
    symbol_table.insert(">=", TokenKind::GreaterEquals);
    symbol_table.insert("+", TokenKind::Plus);
    symbol_table.insert("-", TokenKind::Minus);
    symbol_table.insert("*", TokenKind::Mult);
    symbol_table.insert("/", TokenKind::Div);
    symbol_table.insert("%", TokenKind::Mod);
    symbol_table.insert(";", TokenKind::Semicolon);
    symbol_table.insert("(", TokenKind::OpenParenthesis);
    symbol_table.insert(")", TokenKind::CloseParenthesis);
    symbol_table.insert("{", TokenKind::OpenCurlyBracket);
    symbol_table.insert("}", TokenKind::CloseCurlyBracket);
    symbol_table.insert("[", TokenKind::OpenSquareBracket);
    symbol_table.insert("]", TokenKind::CloseSquareBracket);

    symbol_table
}
