#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Dot,
    DotDot,
    Colon,
    ColonColon,
    Semicolon,
    Comma,

    Plus,
    Minus,
    Slash,
    Star,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,

    Assign,
    Compare,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Hash,
    BangHash,

    NumberLiteral(i64),
    StringLiteral(String),
    CharacterLiteral(char),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Identifier(String),
    Null,

    Fn,
    In,
    For,
    While,
    Return,
    Break,
    Continue,
    Get,
    From,
    Or,
    And,
    If,
    Else,

    Const,
    Dec,
    Int,
    Float,
    Bool,
    String,
    Char,
    Array,

    Arrow,

    Newline,

    Eof,
}

use crate::utils::span::Span;

pub struct Token {
    pub token: TokenType,
    pub line: usize,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, line: usize, span: Span) -> Self {
        Token {
            token,
            lexeme,
            line,
            span,
        }
    }
}
