use crate::lexer::tokentypes::{Token, TokenType};

/// converts raw text from source file into tokens
///
/// it operates character by character and groups them into tokens
pub struct Tokenizer {
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
    /// lexes a source string into Vec<Token>
    ///
    /// appends and TokenType::Eof so parser work with clean list of tokens
    pub fn lex(source: &str) -> Vec<super::tokentypes::Token> {
        let mut lexer = Tokenizer {
            source: source.chars().collect(),
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
        lexer
            .tokens
            .push(Token::new(TokenType::Eof, String::new(), lexer.line));

        log::debug!("Recognized {} token(s)", lexer.tokens.len());
        lexer.tokens
    }
}
