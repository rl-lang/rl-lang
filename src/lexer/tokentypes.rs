//! [`Token`] and [`TokenType`] - the complete vocabulary of the lexer.
//!
//! Every variant the lexer can produce is defined here. Literal-carrying
//! variants (`NumberLiteral`, `StringLiteral`, etc.) embed their parsed value
//! directly so downstream passes never need to re-parse raw text.
use crate::utils::span::Span;

/// Represents every token type the lexer can produce.
///
/// Variants are grouped into:
/// - **Delimiters** - brackets, braces, parens
/// - **Punctuation** - dots, colons, commas, semicolons
/// - **Operators** - arithmetic, comparison, assignment, logical
/// - **Literals** - carry their parsed value directly
/// - **Keywords** - reserved words of the language
/// - **Special** - [`TokenType::Newline`], [`TokenType::Eof`]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // -- delimiters --
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // -- punctuation --
    Dot,
    DotDot,
    Colon,
    ColonColon,
    Semicolon,
    Comma,

    // -- arithmetic --
    Plus,
    Minus,
    Slash,
    Star,

    // -- compound assignment --
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,

    // -- assignment & comparison --
    Assign,
    Compare,

    // -- logical --
    Bang,
    BangEqual,
    BangHash,
    Or,
    And,

    // -- relational --
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // -- special operators --
    Hash,
    Arrow,

    // -- literals --
    /// A 64-bit signed integer e.g. `1000`
    NumberLiteral(i64),
    /// A single byte (u8) e.g. `1`
    ByteLiteral(u8),
    /// A UTF-8 string e.g. `"hello"`
    StringLiteral(String),
    /// A single character e.g. `'a'`
    CharacterLiteral(char),
    /// A 64-bit float e.g. `3.14`
    FloatLiteral(f64),
    /// `true` or `false`
    BoolLiteral(bool),

    // -- identifiers --
    /// Any user-defined name e.g. `foo`, `my_var`
    Identifier(String),

    // -- keywords --
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
    If,
    Else,
    Const,
    Dec,

    // -- type keywords --
    Int,
    Float,
    Bool,
    String,
    Byte,
    Char,
    Array,

    // -- special --
    /// Emitted for each newline in the source
    Newline,
    /// Always the last token in the stream
    Eof,
}

/// A single token produced by the lexer.
///
/// Carries the token type, the original source text ([`Token::lexeme`]),
/// the line it appeared on, and a [`Span`] for error reporting.
pub struct Token {
    /// The classified token type, with literal values inlined for literal variants.
    pub token: TokenType,
    /// The line number in the source file (1-indexed).
    pub line: usize,
    /// The raw source text that produced this token.
    pub lexeme: String,
    /// Byte offsets into the source for error reporting.
    pub span: Span,
}

impl Token {
    /// Creates a new [`Token`].
    pub fn new(token: TokenType, lexeme: String, line: usize, span: Span) -> Self {
        Token {
            token,
            lexeme,
            line,
            span,
        }
    }
}
