use crate::{
    lexer::tokentypes::TokenType, parser::parser_logic::Parser, utils::span::Span,
};

impl Parser {
    /// span of the token currently being looked at (peek position).
    pub fn peek_span(&self) -> Span {
        self.tokens[self.current].span
    }

    /// span of the most recently consumed token.
    pub fn previous_span(&self) -> Span {
        if self.current == 0 {
            self.tokens[0].span
        } else {
            self.tokens[self.current - 1].span
        }
    }

    /// checks the current token if it is end of file ([`TokenType::Eof`]) or not
    ///
    /// if Eof -> true which indicates the last token
    pub fn is_at_end(&self) -> bool {
        if matches!(self.peek(), TokenType::Eof) {
            log::debug!("countered token [TokenType::Eof] indicating end of tokens for the file");
        }
        matches!(self.peek(), TokenType::Eof)
    }

    /// increases the current token been parsed counter
    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
            log::debug!("advancing the parser current token: {}", self.current);
        }
    }

    /// returns [`TokenType`] of current token without consuming it
    pub fn peek(&self) -> TokenType {
        log::debug!(
            "returning current token: [{:?}]",
            &self.tokens[self.current].token
        );
        self.tokens[self.current].token.clone()
    }

    /// returns the previous [`TokenType`] that got consumed
    pub fn previous(&self) -> TokenType {
        log::debug!(
            "returning previous token: [{:?}]",
            &self.tokens[self.current - 1].token
        );
        self.tokens[self.current - 1].token.clone()
    }

    /// checks given [`TokenType`] aginst the current token via `self.peek()`
    ///
    /// if matches returns true else false
    /// doesn't consume the token
    pub fn check(&self, token_type: &TokenType) -> bool {
        let current = self.peek();
        match (token_type, &current) {
            (TokenType::NumberLiteral(_), TokenType::NumberLiteral(_)) => true,
            (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
            (TokenType::FloatLiteral(_), TokenType::FloatLiteral(_)) => true,
            (TokenType::BoolLiteral(_), TokenType::BoolLiteral(_)) => true,
            (TokenType::CharacterLiteral(_), TokenType::CharacterLiteral(_)) => true,
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            _ => token_type == &current,
        }
    }

    /// given a list of [`TokenType`] it match the current token aginst it
    ///
    /// if the token matches the one of the [`TokenType`]s in the list it returns
    /// true while consuming the token
    pub fn match_type(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                log::debug!("Token {:?} matched one in [{:?}]", self.peek(), types);
                self.advance();
                return true;
            }
        }
        log::debug!("Token {:?} did not match any in [{:?}]", self.peek(), types);
        false
    }
}
