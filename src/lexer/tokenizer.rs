use crate::lexer::tokentypes::{Token, TokenType};
use crate::utils::span::Span;

/// converts raw text from source file into tokens
///
/// it operates character by character and groups them into tokens
pub struct Tokenizer {
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
    /// lexes a source string into [`Vec<Token>`]
    ///
    /// appends and TokenType::Eof so parser work with clean list of tokens
    pub fn lex(source: &str) -> Vec<super::tokentypes::Token> {
        let chars: Vec<char> = source.chars().collect();
        let mut byte_offsets = Vec::with_capacity(chars.len() + 1);
        let mut offset = 0usize;
        for c in &chars {
            byte_offsets.push(offset);
            offset += c.len_utf8();
        }
        byte_offsets.push(offset);

        let mut lexer = Tokenizer {
            source: chars,
            byte_offsets,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        };

        // if not end of file scan those characters into tokens and add them to the holder
        while !lexer.is_at_end() {
            // reset lexer position to the current character
            lexer.start = lexer.current;

            lexer.scan_tokens();
        }

        // to mark the end of file when parsing those tokens
        let eof_offset = *lexer.byte_offsets.last().unwrap();
        lexer.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            lexer.line,
            Span::new(eof_offset, eof_offset),
        ));

        log::debug!("Recognized {} token(s)", lexer.tokens.len());
        lexer.tokens
    }

    /// Span covering the current token (from `self.start` to `self.current` in chars).
    pub fn current_span(&self) -> Span {
        Span::new(self.byte_offsets[self.start], self.byte_offsets[self.current])
    }
}
