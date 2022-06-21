use std::fmt::Debug;

#[derive(Debug)]
pub enum TokenType {
    RightCurlyBracket,
    LeftCurlyBracket,
    RightSquareBracket,
    LeftSquareBracket,
    LeftParenthesis,
    RightParenthesis,
    Colon,
    Semicolon,
    Dot,
    Question,
    Coma,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    DoubleEqual,
    Slash,

    Plus,
    Minus,
    Star,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}
