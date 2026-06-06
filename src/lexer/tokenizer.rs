use crate::lexer::tokentypes::{Token, TokenType};
use crate::utils::errors::Error;
use crate::utils::source::SourceFile;
use crate::utils::span::Span;

/// converts raw text from source file into tokens
///
/// it operates character by character and groups them into tokens
pub struct Tokenizer {
    /// the source file (text + name) for error reports
    pub source_file: SourceFile,
    // the source if characters sequence
    pub source: Vec<char>,
    // the accumlated token list
    pub tokens: Vec<super::tokentypes::Token>,
    /// the index of current character
    pub current: usize,
    /// the index of current token
    pub start: usize,
    /// current line number and is incremented every \n
    pub line: usize,
}

impl Tokenizer {
    /// lexes a [`SourceFile`] into [`Vec<Token>`]
    ///
    /// appends a TokenType::Eof so the parser works with a clean list of tokens.
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

        log::debug!("Recognized {} token(s)", lexer.tokens.len());
        Ok(lexer.tokens)
    }

    /// Span covering the current token, in character indices into the source.
    ///
    /// Ariadne's `Source::from(&str)` indexes by character, so spans must be
    /// char-indexed too — passing byte offsets misaligns reports whenever
    /// multi-byte characters precede the span.
    pub fn current_span(&self) -> Span {
        Span::new(self.start, self.current)
    }

    /// build a [`Reason::Lexer`] error anchored at `span`, with the source attached.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        Error::at(crate::utils::errors::Reason::Lexer, message, span)
            .with_source_file(&self.source_file)
    }
}
