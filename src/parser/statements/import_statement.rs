use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_import(&mut self, start: crate::utils::span::Span) -> Result<Statement, Error> {
        // target would be
        // use println, print from std::display
        // which means use will be consumed when found by parser
        // println, print will be identifiers and be thrown in the Vec<String>
        // std::display will be separated to std display and thrown in another Vec<String>
        // checks for comma and coloncolon ummm might make loop

        let first = match self.peek() {
            TokenType::Identifier(name) => name,
            _ => return Err(self.err("expected identifier after 'get'", self.peek_span())),
        };
        self.advance();

        if self.match_type(&[TokenType::ColonColon]) {
            let mut segments = vec![first];
            loop {
                match self.peek() {
                    TokenType::Identifier(seg) => {
                        self.advance();
                        segments.push(seg);
                    }

                    _ => return Err(self.err("expected identifier after '::'", self.peek_span())),
                }
                if !self.match_type(&[TokenType::ColonColon]) {
                    break;
                }
            }

            let name = segments.pop().unwrap();
            let span = start.join(self.previous_span());
            return Ok(Statement::new(
                StatementKind::Import {
                    names: vec![name],
                    path: segments,
                },
                span,
            ));
        }

        let mut names = vec![first];
        if self.match_type(&[TokenType::Comma]) {
            loop {
                match self.peek() {
                    TokenType::Identifier(name) => {
                        // consuming the token
                        self.advance();
                        // adding the target to names list
                        names.push(name);
                    }
                    _ => return Err(self.err("expected identifier after 'get'", self.peek_span())),
                }

                if !self.match_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        if !self.match_type(&[TokenType::From]) {
            return Err(self.err("expected 'from' after names", self.peek_span()));
        }

        let mut path = Vec::new();
        loop {
            match self.peek() {
                TokenType::Identifier(segment) => {
                    // consuming the token
                    self.advance();
                    // adding the target to names list
                    path.push(segment);
                }
                _ => return Err(self.err("expected path after 'from'", self.peek_span())),
            }

            if !self.match_type(&[TokenType::ColonColon]) {
                break;
            }
        }

        let span = start.join(self.previous_span());
        Ok(Statement::new(StatementKind::Import { names, path }, span))
    }
}
