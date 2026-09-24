//! [`Token`] and [`TokenType`] - the complete vocabulary of the lexer.
//!
//! Every variant the lexer can produce is defined here. Literal-carrying
//! variants (`NumberLiteral`, `StringLiteral`, etc.) embed their parsed value
//! directly so downstream passes never need to re-parse raw text.
use rl_utils::span::Span;

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
    Question,

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
    /// Type test operator (`x is int`). English only for now; the
    /// Arabic alias lands separately.
    Is,

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
    FatArrow,
    Wildcard,
    Pipe,

    // -- literals --
    /// A 64-bit unsigned integer e.g. `1000`
    NumberLiteral(u64),
    /// A 64-bit signed integer e.g. `-1000`
    SignedLiteral(i64),
    /// A single byte (u8) e.g. `1`
    ByteLiteral(u8),
    /// A single signed byte (i8) via suffix e.g. `1_i8`
    SignedByteLiteral(i8),
    /// A big byte (u16) via suffix e.g. `1_u16`
    BigByteLiteral(u16),
    /// A big signed byte (i16) via suffix e.g. `1_i16`
    BigSignedByteLiteral(i16),
    /// A small int (i32) via suffix e.g. `1_i32`
    SmallIntLiteral(i32),
    /// A small uint (u32) via suffix e.g. `1_u32`
    SmallUIntLiteral(u32),
    /// A small float (f32) via suffix e.g. `3.14_f32`
    SmallFloatLiteral(f32),
    /// A 64-bit unsigned integer via suffix e.g. `10_u64`
    UIntLiteral(u64),
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
    As,
    Ok,
    Err,
    Match,
    Record,
    Impl,
    Tag,
    Loop,

    // -- type keywords --
    Int,
    UInt,
    Float,
    Bool,
    String,
    Byte,
    SByte,
    Char,
    Array,
    Error,
    Result,
    Map,
    Set,
    Handle,
    Type,

    // -- type modifiers --
    Big,
    Small,

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
    pub leading_trivia: Vec<Trivia>,
    pub trailing_trivia: Vec<Trivia>,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.token, self.lexeme)
    }
}

impl Token {
    /// Creates a new [`Token`].
    pub fn new(token: TokenType, lexeme: String, line: usize, span: Span) -> Self {
        Token {
            token,
            lexeme,
            line,
            span,
            leading_trivia: Vec::new(),
            trailing_trivia: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Trivia {
    LineComment(String),
    BlockComment(String),
    DocComment(String),
    BlankLine,
}
