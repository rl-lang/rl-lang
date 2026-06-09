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
    pub fn parse_for(&mut self, start: crate::utils::span::Span) -> Result<Statement, Error> {
        if matches!(self.peek(), TokenType::LeftBracket) {
            self.advance();
            let init_start = self.peek_span();
            let initializer = Box::new(self.parse_variable_declartion(init_start)?);
            self.match_type(&[TokenType::Comma]);
            let condition = self.parse_expression()?;
            self.match_type(&[TokenType::Comma]);
            let increment = self.parse_expression()?;
            self.match_type(&[TokenType::RightBracket]);
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
            let ident_expr = self.parse_expression()?;
            let variable_name = match ident_expr.kind {
                ExpressionKind::Identifier(name) => name,
                _ => return Err(self.err("for-range expects identifier", ident_expr.span)),
            };
            self.match_type(&[TokenType::In]);

            let range = if matches!(self.peek(), TokenType::NumberLiteral(_)) {
                let start_expr = self.parse_expression()?;
                let range_start = match start_expr.kind {
                    ExpressionKind::Integer(i) => i,
                    _ => return Err(self.err("range should be integers only", start_expr.span)),
                };
                self.match_type(&[TokenType::DotDot]);
                let end_expr = self.parse_expression()?;
                let range_end = match end_expr.kind {
                    ExpressionKind::Integer(i) => i,
                    _ => return Err(self.err("range should be integers only", end_expr.span)),
                };
                let range_vec: Vec<i64> = (range_start..range_end).collect();
                let span = start.join(self.previous_span());
                Box::new(Statement::new(StatementKind::Range(range_vec), span))
            } else if self.match_type(&[TokenType::LeftBracket]) {
                let mut items = Vec::new();
                while self.peek() != TokenType::RightBracket {
                    let value = self.parse_expression()?;
                    items.push(value);
                    if self.peek() == TokenType::RightBracket {
                        break;
                    }
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err("expected ',' between list items", self.peek_span()));
                    }
                }
                self.match_type(&[TokenType::RightBracket]);
                let mut iterable_list = Vec::new();
                for item in items {
                    match item.kind {
                        ExpressionKind::Integer(i) => iterable_list.push(i),
                        _ => return Err(self.err("list items must be integers", item.span)),
                    }
                }
                let span = start.join(self.previous_span());
                Box::new(Statement::new(StatementKind::Range(iterable_list), span))
            } else {
                if matches!(self.peek(), TokenType::Identifier(_)) {
                    let iterable_expression = self.parse_expression()?;
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
