use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub lex: Option<String>,
    pub kind: TokenKind,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier,
    Integer,
    IntegerValue,
    Float,
    FloatValue,
    Char,
    CharValue,
    String,
    StringValue,
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
}

pub fn create_symbol_table<'a>() -> HashMap<&'a str, TokenKind> {
    let mut symbol_table = HashMap::new();

    symbol_table.insert("int", TokenKind::Integer);
    symbol_table.insert("float", TokenKind::Float);
    symbol_table.insert("char", TokenKind::Char);
    symbol_table.insert("string", TokenKind::String);
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
