use super::super::tokenizer::Tokenizer;
use super::super::tokentypes::{Token, TokenType};

impl Tokenizer {
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // advances and returns the next character if no characters return \0 null
    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    pub fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    pub fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let character: char = self.source[self.current];
        self.current += 1;
        character
    }

    pub fn add_token(&mut self, tokentype: TokenType) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        let span = self.current_span();
        self.tokens
            .push(Token::new(tokentype, lexeme, self.line, span));
    }
}
