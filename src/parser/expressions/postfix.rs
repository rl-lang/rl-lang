use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses zero or more postfix method-call chains on `expr`.
    ///
    /// After any primary or sub-expression is built, this function consumes
    /// repeated `.method(args)` suffixes, producing a left-associative chain of
    /// [`ExpressionKind::MethodCall`] nodes. Method names may themselves be
    /// namespaced (`obj.module::method()`).
    ///
    /// Returns `expr` unchanged when the next token is not `.`.
    ///
    /// # Errors
    /// Returns an error if `.` is not followed by a valid identifier, or if the
    /// argument list is not opened with `(`.
    pub fn parse_postfix(&mut self, mut expr: ExprId, start: Span) -> Result<ExprId, Error> {
        loop {
            if self.match_type(&[TokenType::Dot]) {
                if !self.match_type(&[TokenType::Identifier(String::new())]) {
                    return Err(self.err("expected method name after '.'", self.peek_span()));
                }
                let first = if let TokenType::Identifier(name) = self.previous() {
                    name
                } else {
                    return Err(self.err("expected method name after '.'", self.peek_span()));
                };

                let mut method = vec![first];
                while self.match_type(&[TokenType::ColonColon]) {
                    if !self.match_type(&[TokenType::Identifier(String::new())]) {
                        return Err(
                            self.err("expected identifier name after '::'", self.peek_span())
                        );
                    }
                    if let TokenType::Identifier(seg) = self.previous() {
                        method.push(seg);
                    }
                }

                if !self.match_type(&[TokenType::LeftParen]) {
                    return Err(self.err("expected '(' after method name", self.peek_span()));
                }
                while self.match_type(&[TokenType::Newline]) {}
                let mut args = Vec::new();
                if self.peek() != TokenType::RightParen {
                    loop {
                        args.push(self.parse_expression()?);
                        while self.match_type(&[TokenType::Newline]) {}
                        if !self.match_type(&[TokenType::Comma]) {
                            break;
                        }
                        while self.match_type(&[TokenType::Newline]) {}
                    }
                }
                while self.match_type(&[TokenType::Newline]) {}
                self.match_type(&[TokenType::RightParen]);
                let span = start.join(self.previous_span());
                expr = self.ast.alloc_expr(
                    ExpressionKind::MethodCall {
                        caller: expr,
                        method,
                        args,
                    },
                    span,
                );
            } else if self.match_type(&[TokenType::As]) {
                let span = self.previous_span();
                let target_type = self
                    .parse_type(true)
                    .map_err(|_| self.err("expected type after `as`", span))?;
                let span = start.join(self.previous_span());
                expr = self.ast.alloc_expr(
                    ExpressionKind::Cast {
                        value: expr,
                        target_type,
                    },
                    span,
                )
            } else if self.match_type(&[TokenType::Question]) {
                let span = start.join(self.previous_span());
                expr = self.ast.alloc_expr(ExpressionKind::Propagate(expr), span);
            } else {
                break;
            }
        }
        Ok(expr)
    }
}
