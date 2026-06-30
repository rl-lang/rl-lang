//! `for` loop parser - three distinct syntaxes.
//!
//! rl-lang supports three `for` forms:
//!
//! ```text
//! // 1. C-style: initializer, condition, increment inside brackets
//! for [int i = 0, i < 10, i += 1] { … }
//!
//! // 2. Range iteration: integer variable over a literal range
//! for x in 0..10 { … }
//!
//! // 3. Foreach: variable over an array variable or expression
//! for item in my_array { … }
//! ```
//!
//! The leading token after `for` disambiguates the form:
//! - `[` -> C-style ([`StatementKind::For`])
//! - `identifier` -> range or foreach ([`StatementKind::ForRange`] / [`StatementKind::ForEach`])
//! - anything else -> error

use crate::{
    ast::{
        nodes::ExpressionKind,
        statements::{Statement, StatementKind},
    },
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses a `for` statement in one of its three forms.
    ///
    /// Called after `for` has been consumed. Dispatches on the next token:
    ///
    /// - **`[`** - C-style loop. Reads `[T i = init, condition, increment]`
    ///   then a body block. Produces [`StatementKind::For`].
    ///
    /// - **identifier** - inspects the token after the identifier name:
    ///   - followed by `in N..M` or `in [items]` -> [`StatementKind::ForRange`]
    ///     with a pre-evaluated [`StatementKind::Range`] of `i64` values.
    ///   - followed by `in <identifier>` -> [`StatementKind::ForEach`], iterating
    ///     over an array expression at runtime.
    ///
    /// # Errors
    /// Returns an error for invalid range elements (non-integer), missing
    /// commas in C-style headers, or unrecognised `for` syntax.
    pub fn parse_for(&mut self, start: crate::utils::span::Span) -> Result<Statement, Error> {
        if matches!(self.peek(), TokenType::LeftBracket) {
            // C-style: for [T i = init, cond, incr] { … }
            self.advance();
            while self.match_type(&[TokenType::Newline]) {}
            let init_start = self.peek_span();
            let initializer = Box::new(self.parse_variable_declartion(init_start)?);
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::Comma]);
            while self.match_type(&[TokenType::Newline]) {}
            let condition = self.parse_expression()?;
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::Comma]);
            while self.match_type(&[TokenType::Newline]) {}
            let increment = self.parse_expression()?;
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::RightBracket]);
            while self.match_type(&[TokenType::Newline]) {}
            let body = self.parse_block()?;
            let span = start.join(self.previous_span());
            Ok(Statement::new(
                StatementKind::For {
                    initializer,
                    condition,
                    increment,
                    body,
                },
                span,
            ))
        } else if matches!(self.peek(), TokenType::Identifier(_)) {
            // range or foreach: for <ident> in …
            while self.match_type(&[TokenType::Newline]) {}
            let ident_expr = self.parse_expression()?;
            let variable_name = match ident_expr.kind {
                ExpressionKind::Identifier(name) => name,
                _ => return Err(self.err("for-range expects identifier", ident_expr.span)),
            };
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::In]);

            while self.match_type(&[TokenType::Newline]) {}
            let range = if (matches!(self.peek(), TokenType::NumberLiteral(_))
                || (matches!(self.peek(), TokenType::ByteLiteral(_))))
            {
                // literal range: N..M  (integers or bytes, evaluated at parse time)
                let start_expr = self.parse_expression()?;
                while self.match_type(&[TokenType::Newline]) {}
                let range_start = match start_expr.kind {
                    ExpressionKind::Integer(i) => i,
                    ExpressionKind::Byte(b) => b as i64,
                    _ => return Err(self.err("range should be integers only", start_expr.span)),
                };
                self.match_type(&[TokenType::DotDot]);
                let end_expr = self.parse_expression()?;
                let range_end = match end_expr.kind {
                    ExpressionKind::Integer(i) => i,
                    ExpressionKind::Byte(b) => b as i64,
                    _ => return Err(self.err("range should be integers only", end_expr.span)),
                };
                let range_vec: Vec<i64> = (range_start..range_end).collect();
                let span = start.join(self.previous_span());
                Box::new(Statement::new(StatementKind::Range(range_vec), span))
            } else if self.match_type(&[TokenType::LeftBracket]) {
                // inline array literal: [1, 2, 3] (integers only, evaluated at parse time)
                let mut items = Vec::new();
                while self.match_type(&[TokenType::Newline]) {}
                while self.peek() != TokenType::RightBracket {
                    let value = self.parse_expression()?;
                    items.push(value);
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.peek() == TokenType::RightBracket {
                        break;
                    }
                    while self.match_type(&[TokenType::Newline]) {}
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err("expected ',' between list items", self.peek_span()));
                    }
                }
                while self.match_type(&[TokenType::Newline]) {}
                self.match_type(&[TokenType::RightBracket]);
                let mut iterable_list = Vec::new();
                for item in items {
                    match item.kind {
                        ExpressionKind::Integer(i) => iterable_list.push(i),
                        ExpressionKind::Byte(b) => iterable_list.push(b as i64),
                        _ => return Err(self.err("list items must be integers", item.span)),
                    }
                }
                let span = start.join(self.previous_span());
                Box::new(Statement::new(StatementKind::Range(iterable_list), span))
            } else {
                if matches!(self.peek(), TokenType::Identifier(_)) {
                    // foreach: for item in some_array_var_or_expr
                    while self.match_type(&[TokenType::Newline]) {}
                    let iterable_expression = self.parse_expression()?;
                    while self.match_type(&[TokenType::Newline]) {}
                    let body = self.parse_block()?;
                    let span = start.join(self.previous_span());
                    return Ok(Statement::new(
                        StatementKind::ForEach {
                            variable: variable_name,
                            iterable: iterable_expression,
                            body,
                        },
                        span,
                    ));
                }
                return Err(self.err(
                    "expected range (e.g. 1..10), array literal ([1, 2, 3], or array variable",
                    self.peek_span(),
                ));
            };

            while self.match_type(&[TokenType::Newline]) {}
            let body = self.parse_block()?;
            let span = start.join(self.previous_span());
            Ok(Statement::new(
                StatementKind::ForRange {
                    variable: variable_name,
                    range,
                    body,
                },
                span,
            ))
        } else {
            Err(self.err("wrong usage of for", self.peek_span()))
        }
    }
}
