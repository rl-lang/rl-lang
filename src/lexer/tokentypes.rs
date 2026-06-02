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

    Newline,

    Eof,
}

pub struct Token {
    pub token: TokenType,
    pub line: usize,
    pub lexeme: String,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token,
            lexeme,
            line,
        }
    }
}
