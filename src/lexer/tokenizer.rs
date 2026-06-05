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
    /// maps a char index to its byte offset in the original source string.
    /// length is `source.len() + 1`; the final entry is the total byte length.
    pub byte_offsets: Vec<usize>,
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
        let mut byte_offsets = Vec::with_capacity(chars.len() + 1);
        let mut offset = 0usize;
        for c in &chars {
            byte_offsets.push(offset);
            offset += c.len_utf8();
        }
        byte_offsets.push(offset);

        let mut lexer = Tokenizer {
            source_file,
            source: chars,
            byte_offsets,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        };

        while !lexer.is_at_end() {
            lexer.start = lexer.current;
            lexer.scan_tokens()?;
        }

        let eof_offset = *lexer.byte_offsets.last().unwrap();
        lexer.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            lexer.line,
            Span::new(eof_offset, eof_offset),
        ));

        log::debug!("Recognized {} token(s)", lexer.tokens.len());
        Ok(lexer.tokens)
    }

    /// Span covering the current token (from `self.start` to `self.current` in chars).
    pub fn current_span(&self) -> Span {
        Span::new(self.byte_offsets[self.start], self.byte_offsets[self.current])
    }

    /// build a [`Reason::Lexer`] error anchored at `span`, with the source attached.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        Error::at(crate::utils::errors::Reason::Lexer, message, span)
            .with_source_file(&self.source_file)
    }
}
