use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses a primary expression - the highest-precedence, non-recursive forms.
    ///
    /// Handles (in order):
    /// - **Identifiers** - plain names, module paths (`a::b::c`), function calls
    ///   (`f(args)`), variable assignments (`x = expr`), index access (`arr[i]`),
    ///   chained index access (`arr[i][j]`), index-assign (`arr[i] = expr`), and
    ///   call-on-index (`fns[0](args)`).
    /// - **Array literals** - `[a, b, c]`
    /// - **Integer literals** - `42`
    /// - **Byte literals** - `0b` style byte values
    /// - **String literals** - `"hello"`
    /// - **Character literals** - `'x'`
    /// - **Boolean literals** - `true` / `false`
    /// - **Float literals** - `3.14`
    /// - **Null** - the `null` keyword
    /// - **Grouped expressions** - `(expr)`
    /// - **Lambda expressions** - `fn(params) -> T { body }`
    ///
    /// Every matched form is then passed through [`parse_postfix`] to handle
    /// any trailing method-call chains.
    ///
    /// # Errors
    /// Returns an error with the message `"expected expression"` if none of the
    /// above forms match the current token.
    ///
    /// [`parse_postfix`]: Parser::parse_postfix
    pub fn parse_primary(&mut self) -> Result<ExprId, Error> {
        #[cfg(feature = "debug")]
        log::debug!("current index: {:?}", self.current);
        #[cfg(feature = "debug")]
        log::debug!("current token: {:?}", self.peek());

        let start = self.peek_span();

        if self.match_type(&[TokenType::Identifier(String::new())]) {
            #[cfg(feature = "debug")]
            log::debug!("found identifier");
            let ident_span = self.previous_span();
            if let TokenType::Identifier(first) = self.previous() {
                // consume :: segments to build a module path
                let mut path = vec![first];
                while self.match_type(&[TokenType::ColonColon]) {
                    if !self.match_type(&[TokenType::Identifier(String::new())]) {
                        return Err(self.err("expected identifier after `::`", self.peek_span()));
                    }
                    if let TokenType::Identifier(seg) = self.previous() {
                        path.push(seg);
                    }
                }
                let path_span = start.join(self.previous_span());

                if self.match_type(&[TokenType::LeftParen]) {
                    #[cfg(feature = "debug")]
                    log::debug!("found function call");
                    let mut args = Vec::new();
                    while self.match_type(&[TokenType::Newline]) {}
                    if !(std::mem::discriminant(&self.peek())
                        == std::mem::discriminant(&TokenType::RightParen))
                    {
                        loop {
                            args.push(self.parse_expression()?);
                            while self.match_type(&[TokenType::Newline]) {}
                            if !self.match_type(&[TokenType::Comma]) {
                                break;
                            }
                            while self.match_type(&[TokenType::Newline]) {}
                        }
                    }
                    self.match_type(&[TokenType::RightParen]);
                    let span = start.join(self.previous_span());
                    let expr = self
                        .ast
                        .alloc_expr(ExpressionKind::Call { path, args }, span);
                    return self.parse_postfix(expr, start);
                }

                // module paths are not first-class values
                if path.len() > 1 {
                    return Err(self.err(
                        format!("module path `{}` used as value", path.join("::")),
                        path_span,
                    ));
                }
                let name = path.pop().unwrap();

                if self.match_type(&[TokenType::Assign]) {
                    #[cfg(feature = "debug")]
                    log::debug!("found variable assignment");
                    let value = self.parse_expression()?;
                    let span = start.join(self.expr_span(value));
                    let expr = self
                        .ast
                        .alloc_expr(ExpressionKind::Assign { name, value: value }, span);
                    return self.parse_postfix(expr, start);
                }
                if self.match_type(&[TokenType::LeftBracket]) {
                    let index = self.parse_expression()?;
                    self.match_type(&[TokenType::RightBracket]);
                    let after_index_span = self.previous_span();

                    let target = self
                        .ast
                        .alloc_expr(ExpressionKind::Identifier(name.clone()), ident_span);

                    let mut expr = self.ast.alloc_expr(
                        ExpressionKind::Index { target, index },
                        start.join(after_index_span),
                    );

                    // consume chained indices: arr[0][1][2]
                    while self.peek() == TokenType::LeftBracket {
                        self.advance();
                        let next_index = self.parse_expression()?;
                        self.match_type(&[TokenType::RightBracket]);
                        let span = start.join(self.previous_span());
                        expr = self.ast.alloc_expr(
                            ExpressionKind::Index {
                                target: expr,
                                index: next_index,
                            },
                            span,
                        );
                    }

                    // call on the result of an index: fns[0](arg)
                    if self.match_type(&[TokenType::LeftParen]) {
                        let mut args = Vec::new();
                        while self.match_type(&[TokenType::Newline]) {}
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
                        let expr = self
                            .ast
                            .alloc_expr(ExpressionKind::CallExpr { callee: expr, args }, span);
                        return self.parse_postfix(expr, start);
                    }

                    if self.match_type(&[TokenType::Assign]) {
                        #[cfg(feature = "debug")]
                        log::debug!("found array item assignment");
                        let value = self.parse_expression()?;
                        let span = start.join(self.expr_span(value));
                        if let ExpressionKind::Index { target, index } = self.expr_kind(expr) {
                            let expr = self.ast.alloc_expr(
                                ExpressionKind::IndexAssign {
                                    target,
                                    index,
                                    value: value,
                                },
                                span,
                            );
                            return self.parse_postfix(expr, start);
                        }
                    }

                    return self.parse_postfix(expr, start);
                }
                let expr = self
                    .ast
                    .alloc_expr(ExpressionKind::Identifier(name), ident_span);
                return self.parse_postfix(expr, start);
            }
        }

        if self.match_type(&[TokenType::LeftBracket]) {
            let mut items = Vec::new();
            while self.match_type(&[TokenType::Newline]) {}
            while self.peek() != TokenType::RightBracket {
                items.push(self.parse_expression()?);
                while self.match_type(&[TokenType::Newline]) {}
                if self.peek() == TokenType::RightBracket {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    return Err(self.err("expected `,` between array items", self.peek_span()));
                }
                while self.match_type(&[TokenType::Newline]) {}
            }
            self.match_type(&[TokenType::RightBracket]);
            let span = start.join(self.previous_span());
            let expr = self
                .ast
                .alloc_expr(ExpressionKind::ArrayLiteral(items), span);
            return self.parse_postfix(expr, start);
        }

        if self.match_type(&[TokenType::NumberLiteral(0)]) {
            #[cfg(feature = "debug")]
            log::debug!("found number");
            let span = self.previous_span();
            if let TokenType::NumberLiteral(n) = self.previous() {
                if self.match_type(&[TokenType::As]) {
                    if self.match_type(&[TokenType::Int, TokenType::Byte, TokenType::Float]) {
                        match self.previous() {
                            TokenType::Int => {
                                let expr = self.ast.alloc_expr(ExpressionKind::Integer(n), span);

                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Float => {
                                let expr =
                                    self.ast.alloc_expr(ExpressionKind::Float(n as f64), span);

                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Byte => {
                                if !(0..=255).contains(&n) {
                                    return Err(self
                                        .err(format!("value {} is too large for byte", n), span));
                                }

                                let expr = self.ast.alloc_expr(ExpressionKind::Byte(n as u8), span);
                                return self.parse_postfix(expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    return Err(self.err("expected type after `as`", self.previous_span()));
                }

                let expr = self.ast.alloc_expr(ExpressionKind::Integer(n), span);
                return self.parse_postfix(expr, start);
            }
        }

        if self.match_type(&[TokenType::ByteLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::ByteLiteral(b) = self.previous() {
                if self.match_type(&[TokenType::As]) {
                    if self.match_type(&[TokenType::Int, TokenType::Byte, TokenType::Float]) {
                        match self.previous() {
                            TokenType::Int => {
                                let expr =
                                    self.ast.alloc_expr(ExpressionKind::Integer(b as i64), span);

                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Float => {
                                let expr =
                                    self.ast.alloc_expr(ExpressionKind::Float(b as f64), span);

                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Byte => {
                                if !(0..=255).contains(&b) {
                                    return Err(self
                                        .err(format!("value {} is too large for byte", b), span));
                                }

                                let expr = self.ast.alloc_expr(ExpressionKind::Byte(b), span);

                                return self.parse_postfix(expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    return Err(self.err("expected type after `as`", self.previous_span()));
                }
                let expr = self.ast.alloc_expr(ExpressionKind::Byte(b), span);
                return self.parse_postfix(expr, start);
            }
        }

        if self.match_type(&[TokenType::StringLiteral(String::new())]) {
            #[cfg(feature = "debug")]
            log::debug!("found string");
            let span = self.previous_span();
            if let TokenType::StringLiteral(s) = self.previous() {
                let expr = self.ast.alloc_expr(ExpressionKind::String(s), span);
                return self.parse_postfix(expr, start);
            }
        }

        if matches!(
            self.tokens[self.current].token,
            TokenType::CharacterLiteral(_)
        ) {
            self.advance();
            #[cfg(feature = "debug")]
            log::debug!("found character");
            let span = self.previous_span();
            if let TokenType::CharacterLiteral(c) = self.previous() {
                let expr = self.ast.alloc_expr(ExpressionKind::Character(c), span);
                return self.parse_postfix(expr, start);
            }
        }

        if self.match_type(&[TokenType::BoolLiteral(false)]) {
            let span = self.previous_span();
            if let TokenType::BoolLiteral(b) = self.previous() {
                let expr = self.ast.alloc_expr(ExpressionKind::Bool(b), span);
                return self.parse_postfix(expr, start);
            }
        }

        if self.match_type(&[TokenType::FloatLiteral(0.0)]) {
            #[cfg(feature = "debug")]
            log::debug!("oh no found float");
            let span = self.previous_span();
            if let TokenType::FloatLiteral(f) = self.previous() {
                if self.match_type(&[TokenType::As]) {
                    if self.match_type(&[TokenType::Int, TokenType::Byte, TokenType::Float]) {
                        match self.previous() {
                            TokenType::Int => {
                                let expr =
                                    self.ast.alloc_expr(ExpressionKind::Integer(f as i64), span);

                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Float => {
                                let expr = self.ast.alloc_expr(ExpressionKind::Float(f), span);

                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Byte => {
                                if !(0.0..=255.0).contains(&f) {
                                    return Err(self
                                        .err(format!("value {} is too large for byte", f), span));
                                }

                                let expr = self.ast.alloc_expr(ExpressionKind::Byte(f as u8), span);

                                return self.parse_postfix(expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    return Err(self.err("expected type after `as`", span));
                }
                let expr = self
                    .ast
                    .alloc_expr(ExpressionKind::Float(f), self.previous_span());
                return self.parse_postfix(expr, start);
            }
        }

        if self.match_type(&[TokenType::Error]) {
            let error_start = self.previous_span();
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::LeftParen]) {
                return Err(self.err("expected `(` after `error`", self.peek_span()));
            }
            while self.match_type(&[TokenType::Newline]) {}
            let inner = self.parse_expression()?;
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightParen]) {
                return Err(self.err("expected `)` after error value", self.peek_span()));
            }
            let span = error_start.join(self.previous_span());
            let expr = self
                .ast
                .alloc_expr(ExpressionKind::ErrorLiteral(inner), span);
            return self.parse_postfix(expr, start);
        }

        if self.match_type(&[TokenType::Ok]) {
            let kw_span = self.previous_span();
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::LeftParen]) {
                return Err(self.err("expected `(` after `ok`", self.peek_span()));
            }
            while self.match_type(&[TokenType::Newline]) {}
            let inner = self.parse_expression()?;
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightParen]) {
                return Err(self.err("expected `)` after ok value", self.peek_span()));
            }
            let span = kw_span.join(self.previous_span());

            let expr = self.ast.alloc_expr(ExpressionKind::OkLiteral(inner), span);

            return self.parse_postfix(expr, start);
        }

        if self.match_type(&[TokenType::Err]) {
            let kw_span = self.previous_span();
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::LeftParen]) {
                return Err(self.err("expected `(` after `err`", self.peek_span()));
            }
            while self.match_type(&[TokenType::Newline]) {}
            let inner = self.parse_expression()?;
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightParen]) {
                return Err(self.err("expected `)` after err value", self.peek_span()));
            }
            let span = kw_span.join(self.previous_span());

            let expr = self.ast.alloc_expr(ExpressionKind::ErrLiteral(inner), span);

            return self.parse_postfix(expr, start);
        }

        if self.match_type(&[TokenType::Null]) {
            let span = self.previous_span();
            let expr = self.ast.alloc_expr(ExpressionKind::Null, span);
            return self.parse_postfix(expr, start);
        }

        if self.match_type(&[TokenType::LeftParen]) {
            #[cfg(feature = "debug")]
            log::debug!("found group start");
            while self.match_type(&[TokenType::Newline]) {}
            let first = self.parse_expression()?;
            if self.match_type(&[TokenType::Comma]) {
                // tuple literal: (expr, expr, ...)
                let mut items = vec![first];
                while self.peek() != TokenType::RightParen && self.peek() != TokenType::Eof {
                    while self.match_type(&[TokenType::Newline]) {}
                    items.push(self.parse_expression()?);
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.peek() == TokenType::RightParen {
                        break;
                    }
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err("expected , between tuple elements", self.peek_span()));
                    }
                }
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::RightParen]) {
                    return Err(self.err("expected ) after tuple elements", self.peek_span()));
                }
                let span = start.join(self.previous_span());
                let expr = self
                    .ast
                    .alloc_expr(ExpressionKind::TupleLiteral(items), span);
                return self.parse_postfix(expr, start);
            }
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::RightParen]);
            let span = start.join(self.previous_span());
            let expr = self.ast.alloc_expr(ExpressionKind::Grouping(first), span);
            return self.parse_postfix(expr, start);
        }

        if self.match_type(&[TokenType::Fn]) {
            let lambda_start = self.previous_span();
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::LeftParen]);

            let mut params: Vec<crate::ast::statements::Param> = Vec::new();
            while self.match_type(&[TokenType::Newline]) {}
            while !self.match_type(&[TokenType::RightParen]) {
                let param_type = self.parse_param_type()?;
                while self.match_type(&[TokenType::Newline]) {}
                let param_name = match self.peek() {
                    TokenType::Identifier(n) => {
                        self.advance();
                        n
                    }
                    _ => return Err(self.err("expected parameter name", self.peek_span())),
                };
                params.push(crate::ast::statements::Param {
                    param_name,
                    param_type,
                });
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::Comma]) {
                    break;
                }
            }
            while self.match_type(&[TokenType::Newline]) {}
            self.match_type(&[TokenType::RightParen]);

            while self.match_type(&[TokenType::Newline]) {}
            let return_type = if self.match_type(&[TokenType::Arrow]) {
                while self.match_type(&[TokenType::Newline]) {}
                Some(self.parse_param_type()?)
            } else {
                None
            };

            while self.match_type(&[TokenType::Newline]) {}
            let body = self.parse_block()?;
            let span = lambda_start.join(self.previous_span());
            let expr = self.ast.alloc_expr(
                ExpressionKind::Lambda {
                    params,
                    return_type,
                    body,
                },
                span,
            );
            return self.parse_postfix(expr, start);
        }

        Err(self.err("expected expression", self.peek_span()))
    }
}
