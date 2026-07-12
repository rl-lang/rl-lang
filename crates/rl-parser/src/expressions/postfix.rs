use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

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
            // --- method call / field access ---
            if self.match_type(&[TokenType::Dot]) {
                if !self.match_type(&[TokenType::Identifier(String::new())]) {
                    return Err(self.err("expected name after '.'", self.peek_span()));
                }
                let first = if let TokenType::Identifier(name) = self.previous() {
                    name
                } else {
                    return Err(self.err("expected name after '.'", self.peek_span()));
                };

                // a bare `.name` not followed by `::` or `(` is a field access,
                // not a method call.
                if self.peek() != TokenType::ColonColon && self.peek() != TokenType::LeftParen {
                    let span = start.join(self.previous_span());
                    #[cfg(feature = "debug")]
                    log::trace!(
                        "alloc FieldAccess expr: target={:?} field={:?} @ {:?}",
                        expr,
                        first,
                        span
                    );
                    expr = self.ast_arena.alloc_expr(
                        ExpressionKind::FieldAccess {
                            target: expr,
                            field: first,
                        },
                        span,
                    );

                    // --- field assignment: target.field = value ---
                    if self.match_type(&[TokenType::Assign]) {
                        let value = self.parse_expression()?;
                        let value_id = self.ast_arena.exprs.get(value);
                        let assign_span = start.join(value_id.span);
                        let expr_kind = self.ast_arena.exprs.get(expr).kind.clone();
                        if let ExpressionKind::FieldAccess { target, field } = expr_kind {
                            #[cfg(feature = "debug")]
                            log::trace!(
                                "alloc FieldAssign expr: target={:?} field={:?} value={:?} @ {:?}",
                                target,
                                field,
                                value,
                                assign_span
                            );
                            return Ok(self.ast_arena.alloc_expr(
                                ExpressionKind::FieldAssign {
                                    target,
                                    field,
                                    value,
                                },
                                assign_span,
                            ));
                        }
                    }

                    continue;
                }

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

                #[cfg(feature = "debug")]
                log::trace!(
                    "alloc MethodCall expr: caller={:?} method={:?} args={} @ {:?}",
                    expr,
                    method,
                    args.len(),
                    span
                );

                expr = self.ast_arena.alloc_expr(
                    ExpressionKind::MethodCall {
                        caller: expr,
                        method,
                        args,
                    },
                    span,
                );
            }
            // --- cast ---
            else if self.match_type(&[TokenType::As]) {
                let span = self.previous_span();
                let target_type = self
                    .parse_type(true)
                    .map_err(|_| self.err("expected type after `as`", span))?;
                let span = start.join(self.previous_span());

                #[cfg(feature = "debug")]
                log::trace!(
                    "alloc Cast expr: value={:?} target_type={:?} @ {:?}",
                    expr,
                    target_type,
                    span
                );

                expr = self.ast_arena.alloc_expr(
                    ExpressionKind::Cast {
                        value: expr,
                        target_type,
                    },
                    span,
                )
            }
            // --- propagate ---
            else if self.match_type(&[TokenType::Question]) {
                let span = start.join(self.previous_span());

                #[cfg(feature = "debug")]
                log::trace!("alloc Propagate expr: inner={:?} @ {:?}", expr, span);

                expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::Propagate(expr), span)
            } else {
                break;
            }
        }
        Ok(expr)
    }
}
