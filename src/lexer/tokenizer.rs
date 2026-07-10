//! [`Tokenizer`] struct and the main character-by-character scan loop.
//!
//! Holds the source text, the current byte cursor, and the accumulated token
//! list. The scan loop in here dispatches each character to the appropriate
//! sub-scanner in `types/` or handles single/double-character operators inline.
use crate::lexer::tokentypes::{Token, TokenType, Trivia};
use crate::utils::errors::Error;
use crate::utils::source::SourceFile;
use crate::utils::span::Span;

/// Converts raw source text into a flat list of [`Token`]s.
///
/// Operates character by character, grouping lexemes into tokens.
/// The main entry point is [`Tokenizer::lex`].
pub struct Tokenizer {
    /// The source file (text + name) used for error reporting.
    pub source_file: SourceFile,
    /// The source text as a sequence of characters.
    pub source: Vec<char>,
    /// The accumulated token list built during lexing.
    pub tokens: Vec<super::tokentypes::Token>,
    /// Index of the current character being examined.
    pub current: usize,
    /// Index of the first character of the current token.
    pub start: usize,
    /// Current line number, incremented on every `\n`.
    pub line: usize,
    pub pending_trivia: Vec<Trivia>,
}

impl Tokenizer {
    /// The main entry point: lexes a [`SourceFile`] into a [`Vec<Token>`].
    ///
    /// Drives [`Tokenizer::scan_tokens`] in a loop until the source is exhausted,
    /// then appends a [`TokenType::Eof`] so the parser always has a clean terminator.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the source contains an unrecognized character,
    /// unterminated string, or invalid literal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rl_lang::{
    ///     lexer::{
    ///         tokenizer::Tokenizer,
    ///         tokentypes::TokenType,
    ///     },
    ///     utils::source::SourceFile,
    /// };
    ///
    /// let tokens = match Tokenizer::lex(SourceFile::new("source", "1 == 1".to_string())) {
    ///     Ok(tokens) => tokens,
    ///     Err(error) => {
    ///         error.report_to_stderr();
    ///         std::process::exit(1);
    ///     },
    /// };
    ///
    /// assert_eq!(tokens[0].token, TokenType::NumberLiteral(1));
    /// assert_eq!(tokens[1].token, TokenType::Compare);
    /// assert_eq!(tokens[2].token, TokenType::NumberLiteral(1));
    /// assert_eq!(tokens[3].token, TokenType::Eof);
    /// ```
    pub fn lex(source_file: SourceFile) -> Result<Vec<Token>, Error> {
        let chars: Vec<char> = source_file.text.chars().collect();
        let eof_char_index = chars.len();

        let mut lexer = Tokenizer {
            source_file,
            source: chars,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
            pending_trivia: Vec::new(),
        };

        while !lexer.is_at_end() {
            lexer.start = lexer.current;
            lexer.scan_tokens()?;
        }

        lexer.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            lexer.line,
            Span::new(eof_char_index, eof_char_index),
        ));

        #[cfg(feature = "debug")]
        log::debug!("Recognized {} token(s)", lexer.tokens.len());
        Ok(lexer.tokens)
    }

    /// Returns a [`Span`] covering the current token in character indices.
    ///
    /// Ariadne's `Source::from(&str)` indexes by character, so spans must be
    /// char-indexed - passing byte offsets misaligns reports for multi-byte characters.
    pub fn current_span(&self) -> Span {
        Span::new(self.start, self.current)
    }

    /// Builds a [`Reason::Lexer`] error anchored at `span`.
    ///
    /// Attaches the source file so Ariadne can render the relevant source line
    /// alongside the error message.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        Error::at(crate::utils::errors::Reason::Lexer, message, span)
            .with_source_file(&self.source_file)
    }
}
