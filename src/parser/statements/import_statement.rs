use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_import(&mut self, start: crate::utils::span::Span) -> Result<Statement, Error> {
        // must be an identifier
        let first = match self.peek() {
            TokenType::Identifier(name) => name,
            _ => return Err(self.err("expected identifier after 'get'", self.peek_span())),
        };
        self.advance();

        // get mymodule::utils  OR  get std::math::sin
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
            let span = start.join(self.previous_span());
            let is_std = segments[0] == "std";
            return if is_std {
                let name = segments.pop().unwrap();
                Ok(Statement::new(
                    StatementKind::Import {
                        names: vec![name],
                        path: segments,
                    },
                    span,
                ))
            } else {
                Ok(Statement::new(
                    StatementKind::ImportFile { path: segments },
                    span,
                ))
            };
        }

        // get mymodule  (single segment, no ::, no from)
        if !matches!(self.peek(), TokenType::Comma | TokenType::From) {
            let span = start.join(self.previous_span());
            return Ok(Statement::new(
                StatementKind::ImportFile { path: vec![first] },
                span,
            ));
        }

        // get add, sub from ...
        let mut names = vec![first];
        if self.match_type(&[TokenType::Comma]) {
            loop {
                match self.peek() {
                    TokenType::Identifier(name) => {
                        self.advance();
                        names.push(name);
                    }
                    _ => return Err(self.err("expected identifier after ','", self.peek_span())),
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
                    self.advance();
                    path.push(segment);
                }
                _ => return Err(self.err("expected path after 'from'", self.peek_span())),
            }
            if !self.match_type(&[TokenType::ColonColon]) {
                break;
            }
        }

        let span = start.join(self.previous_span());
        let is_std = path.first().map(|s| s == "std").unwrap_or(false);

        if is_std {
            Ok(Statement::new(StatementKind::Import { names, path }, span))
        } else {
            Ok(Statement::new(
                StatementKind::ImportFileNamed { path, names },
                span,
            ))
        }
    }
}
