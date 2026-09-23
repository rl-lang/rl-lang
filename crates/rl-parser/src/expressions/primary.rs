use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

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

        // ---- identifier start ----
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

                // --- function call ---
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
                    if !self.match_type(&[TokenType::RightParen]) {
                        return Err(self.err("expected `)` after arguments", self.peek_span()));
                    }
                    let span = start.join(self.previous_span());
                    #[cfg(feature = "debug")]
                    log::trace!(
                        "alloc Call expr: path={:?} args={} @ {:?}",
                        path,
                        args.len(),
                        span
                    );
                    let expr = self
                        .ast_arena
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

                // --- struct literal: Name { field: value, ... } ---
                if self.record_names.contains(&name) && self.peek() == TokenType::LeftBrace {
                    return self.parse_struct_literal(name, start);
                }

                // --- enum variant reference: Name.Variant ---
                if self.tag_names.contains(&name) && self.peek() == TokenType::Dot {
                    self.advance();
                    let variant = match self.peek() {
                        TokenType::Identifier(v) => {
                            self.advance();
                            v
                        }
                        _ => {
                            return Err(
                                self.err("expected variant name after `.`", self.peek_span())
                            );
                        }
                    };
                    let span = start.join(self.previous_span());
                    #[cfg(feature = "debug")]
                    log::trace!(
                        "alloc EnumVariant expr: enum_name={:?} variant={:?} @ {:?}",
                        name,
                        variant,
                        span
                    );
                    let expr = self.ast_arena.alloc_expr(
                        ExpressionKind::EnumVariant {
                            enum_name: name,
                            variant,
                        },
                        span,
                    );
                    return self.parse_postfix(expr, start);
                }

                // --- assign expression ---
                if self.match_type(&[TokenType::Assign]) {
                    #[cfg(feature = "debug")]
                    log::debug!("found variable assignment");
                    let value = self.parse_expression()?;
                    let value_id = self.ast_arena.exprs.get(value);
                    let span = start.join(value_id.span);
                    #[cfg(feature = "debug")]
                    log::trace!(
                        "alloc Assign expr: name={} value={:?} @ {:?}",
                        name,
                        value,
                        span
                    );
                    let expr = self
                        .ast_arena
                        .alloc_expr(ExpressionKind::Assign { name, value }, span);

                    return self.parse_postfix(expr, start);
                }

                // --- index ---
                if self.match_type(&[TokenType::LeftBracket]) {
                    #[cfg(feature = "debug")]
                    log::trace!(
                        "alloc Identifier(index target) expr: name={} @ {:?}",
                        name,
                        ident_span
                    );

                    let index = self.parse_expression()?;
                    while self.match_type(&[TokenType::Newline]) {}
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after index", self.peek_span()));
                    }
                    let after_index_span = self.previous_span();

                    let target = self
                        .ast_arena
                        .alloc_expr(ExpressionKind::Identifier(name.clone()), ident_span);

                    #[cfg(feature = "debug")]
                    log::trace!(
                        "alloc Index expr: target={:?} index={:?} @ {:?}",
                        target,
                        index,
                        self.ast_arena.exprs.get(target).span
                    );

                    let mut expr = self.ast_arena.alloc_expr(
                        ExpressionKind::Index { target, index },
                        start.join(after_index_span),
                    );

                    // --- chained indices: arr[0][1][2] ---
                    while self.peek() == TokenType::LeftBracket {
                        self.advance();
                        let next_index = self.parse_expression()?;
                        while self.match_type(&[TokenType::Newline]) {}
                        if !self.match_type(&[TokenType::RightBracket]) {
                            return Err(self.err("expected `]` after index", self.peek_span()));
                        }
                        let span = start.join(self.previous_span());

                        #[cfg(feature = "debug")]
                        log::trace!(
                            "alloc Index(chained) expr: target={:?} index={:?} @ {:?}",
                            expr,
                            next_index,
                            span
                        );

                        expr = self.ast_arena.alloc_expr(
                            ExpressionKind::Index {
                                target: expr,
                                index: next_index,
                            },
                            span,
                        );
                    }

                    // --- call on the result of an index: fns[0](arg) ---
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
                        if !self.match_type(&[TokenType::RightParen]) {
                            return Err(self.err("expected `)` after arguments", self.peek_span()));
                        }
                        let span = start.join(self.previous_span());
                        #[cfg(feature = "debug")]
                        log::trace!(
                            "alloc CallExpr expr: callee={:?} args={} @ {:?}",
                            expr,
                            args.len(),
                            span
                        );

                        let call_expr = self
                            .ast_arena
                            .alloc_expr(ExpressionKind::CallExpr { callee: expr, args }, span);

                        return self.parse_postfix(call_expr, start);
                    }

                    // --- index assign ---
                    if self.match_type(&[TokenType::Assign]) {
                        #[cfg(feature = "debug")]
                        log::debug!("found array item assignment");
                        let value = self.parse_expression()?;

                        let value_id = self.ast_arena.exprs.get(value);
                        let expr_id = self.ast_arena.exprs.get(expr);

                        let span = start.join(value_id.span);
                        if let ExpressionKind::Index { target, index } = expr_id.kind.clone() {
                            let expr = self.ast_arena.alloc_expr(
                                ExpressionKind::IndexAssign {
                                    target,
                                    index,
                                    value,
                                },
                                span,
                            );

                            #[cfg(feature = "debug")]
                            log::trace!(
                                "alloc IndexAssign expr: target={:?} index={:?} value={:?} @ {:?}",
                                target,
                                index,
                                value,
                                span
                            );
                            return self.parse_postfix(expr, start);
                        }
                    }

                    return self.parse_postfix(expr, start);
                }

                // --- identifier ---
                #[cfg(feature = "debug")]
                log::trace!("alloc Identifier expr: name={} @ {:?}", name, ident_span);

                let expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::Identifier(name), ident_span);

                return self.parse_postfix(expr, start);
            }
        }
        // ---- identifier end ----

        // --- map literal ---
        if self.peek() == TokenType::LeftBrace {
            self.advance();
            let mut entries = Vec::new();
            while self.match_type(&[TokenType::Newline]) {}
            while self.peek() != TokenType::RightBrace {
                let key = self.parse_expression()?;
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::Colon]) {
                    return Err(self.err("expected `:` after map key", self.peek_span()));
                }
                while self.match_type(&[TokenType::Newline]) {}
                let value = self.parse_expression()?;
                entries.push((key, value));
                while self.match_type(&[TokenType::Newline]) {}
                if self.peek() == TokenType::RightBrace {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    return Err(self.err("expected `,` between map entries", self.peek_span()));
                }
                while self.match_type(&[TokenType::Newline]) {}
            }
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightBrace]) {
                return Err(self.err("expected `}` after map literal", self.peek_span()));
            }
            let span = start.join(self.previous_span());

            #[cfg(feature = "debug")]
            log::trace!(
                "alloc MapLiteral expr: entries={} @ {:?}",
                entries.len(),
                span
            );

            let expr = self
                .ast_arena
                .alloc_expr(ExpressionKind::MapLiteral(entries), span);
            return self.parse_postfix(expr, start);
        }

        // --- set literal ---
        if self.match_type(&[TokenType::LeftBrace]) {
            let mut items = Vec::new();
            while self.match_type(&[TokenType::Newline]) {}
            while self.peek() != TokenType::RightBrace {
                items.push(self.parse_expression()?);
                while self.match_type(&[TokenType::Newline]) {}
                if self.peek() == TokenType::RightBrace {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    return Err(self.err("expected `,` between set items", self.peek_span()));
                }
                while self.match_type(&[TokenType::Newline]) {}
            }
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightBrace]) {
                return Err(self.err("expected `}` after set literal", self.peek_span()));
            }
            let span = start.join(self.previous_span());

            #[cfg(feature = "debug")]
            log::trace!("alloc SetLiteral expr: items={} @ {:?}", items.len(), span);

            let expr = self
                .ast_arena
                .alloc_expr(ExpressionKind::SetLiteral(items), span);
            return self.parse_postfix(expr, start);
        }

        // --- array literal ---
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
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightBracket]) {
                return Err(self.err("expected `]` after array literal", self.peek_span()));
            }
            let span = start.join(self.previous_span());

            #[cfg(feature = "debug")]
            log::trace!(
                "alloc ArrayLiteral expr: items={} @ {:?}",
                items.len(),
                span
            );

            let expr = self
                .ast_arena
                .alloc_expr(ExpressionKind::ArrayLiteral(items), span);
            return self.parse_postfix(expr, start);
        }

        // ---- numbers start ----
        // --- signed integer ---
        if self.match_type(&[TokenType::SignedLiteral(0)]) {
            #[cfg(feature = "debug")]
            log::debug!("found number <signed integer>");
            let span = self.previous_span();
            if let TokenType::SignedLiteral(n) = self.previous() {
                // ---- cast start ----
                if self.match_type(&[TokenType::As]) {
                    if self.match_type(&[
                        TokenType::Int,
                        TokenType::Byte,
                        TokenType::Float,
                        TokenType::UInt,
                        TokenType::Small,
                        TokenType::Big,
                        TokenType::SByte,
                    ]) {
                        match self.previous() {
                            TokenType::Int => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Integer expr (from signed number, cast to int): {} @ {:?}",
                                    n,
                                    span
                                );
                                let expr =
                                    self.ast_arena.alloc_expr(ExpressionKind::Integer(n), span);
                                return self.parse_postfix(expr, start);
                            }
                            TokenType::UInt => {
                                let expr = match u64::try_from(n) {
                                    Ok(u) => {
                                        self.ast_arena.alloc_expr(ExpressionKind::UInt(u), span)
                                    }
                                    Err(e) => {
                                        return Err(self.err(e.to_string(), span));
                                    }
                                };
                                return self.parse_postfix(expr, start);
                            }
                            TokenType::Small => {
                                if self.match_type(&[TokenType::Float]) {
                                    let expr = match i32::try_from(n) {
                                        Ok(f) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::SFloat(f as f32), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::Int]) {
                                    let expr = match i32::try_from(n) {
                                        Ok(f) => {
                                            self.ast_arena.alloc_expr(ExpressionKind::SInt(f), span)
                                        }
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::UInt]) {
                                    let expr = match u32::try_from(n) {
                                        Ok(u) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::SUInt(u), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'small'", span));
                            }
                            TokenType::Big => {
                                if self.match_type(&[TokenType::Byte]) {
                                    let expr = match u16::try_from(n) {
                                        Ok(b) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::BByte(b), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::SByte]) {
                                    let expr = match i16::try_from(n) {
                                        Ok(b) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::BSByte(b), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'big'", span));
                            }

                            TokenType::Float => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Float expr (from signed number, cast to float): {} @ {:?}",
                                    n as f64,
                                    span
                                );
                                let expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::Float(n as f64), span);
                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Byte => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Byte expr (from signed number, cast to byte): {} @ {:?}",
                                    n as u8,
                                    span
                                );
                                let expr = match u8::try_from(n) {
                                    Ok(b) => {
                                        self.ast_arena.alloc_expr(ExpressionKind::Byte(b), span)
                                    }
                                    Err(e) => {
                                        return Err(self.err(e.to_string(), span));
                                    }
                                };
                                return self.parse_postfix(expr, start);
                            }
                            TokenType::SByte => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Byte expr (from signed number, cast to sbyte): {} @ {:?}",
                                    n as u8,
                                    span
                                );
                                let expr = match i8::try_from(n) {
                                    Ok(b) => {
                                        self.ast_arena.alloc_expr(ExpressionKind::SByte(b), span)
                                    }
                                    Err(e) => {
                                        return Err(self.err(e.to_string(), span));
                                    }
                                };
                                return self.parse_postfix(expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float/uint types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    if self.match_type(&[TokenType::UInt]) {
                        return Err(
                            self.err(format!("cannot cast signed value {} to uint", n), span)
                        );
                    }
                    // `as <alias>`: no constant folding (targets are
                    // arbitrary); emit a regular checked cast instead.
                    if let TokenType::Identifier(alias) = self.peek() {
                        if let Some(target) =
                            self.ast_arena.type_aliases.get(&alias).cloned()
                        {
                            self.advance();
                            self.ast_arena
                                .alias_uses
                                .push((alias, self.previous_span()));
                            let value = self
                                .ast_arena
                                .alloc_expr(ExpressionKind::Integer(n), span);
                            let cast_span = start.join(self.previous_span());
                            let expr = self.ast_arena.alloc_expr(
                                ExpressionKind::Cast {
                                    value,
                                    target_type: target,
                                },
                                cast_span,
                            );
                            return self.parse_postfix(expr, start);
                        }
                    }
                    return Err(self.err("expected type after `as`", self.previous_span()));
                }
                // ---- cast end ----

                // no cast logic - a signed literal is always `int`
                #[cfg(feature = "debug")]
                log::trace!(
                    "alloc Integer expr (plain signed literal): {} @ {:?}",
                    n,
                    span
                );
                let expr = self.ast_arena.alloc_expr(ExpressionKind::Integer(n), span);
                return self.parse_postfix(expr, start);
            }
        }

        // --- integer ---
        if self.match_type(&[TokenType::NumberLiteral(0)]) {
            #[cfg(feature = "debug")]
            log::debug!("found number <integer>");
            let span = self.previous_span();
            if let TokenType::NumberLiteral(n) = self.previous() {
                // ---- cast start ----
                if self.match_type(&[TokenType::As]) {
                    // from integer to T
                    if self.match_type(&[
                        TokenType::Int,
                        TokenType::Byte,
                        TokenType::Float,
                        TokenType::UInt,
                        TokenType::Small,
                        TokenType::Big,
                        TokenType::SByte,
                    ]) {
                        match self.previous() {
                            TokenType::Int => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Integer expr (from number, cast to int): {} @ {:?}",
                                    n,
                                    span
                                );

                                let Ok(v) = i64::try_from(n) else {
                                    return Err(self.err(
                                        format!(
                                            "value {} is out of range for int ({}..={}) - use `as uint`",
                                            n,
                                            i64::MIN,
                                            i64::MAX
                                        ),
                                        span,
                                    ));
                                };
                                let expr =
                                    self.ast_arena.alloc_expr(ExpressionKind::Integer(v), span);
                                return self.parse_postfix(expr, start);
                            }

                            TokenType::UInt => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc UInt expr (from number, cast to uint): {} @ {:?}",
                                    n,
                                    span
                                );
                                let expr = self.ast_arena.alloc_expr(ExpressionKind::UInt(n), span);
                                return self.parse_postfix(expr, start);
                            }

                            TokenType::Float => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Float expr (from number, cast to float): {} @ {:?}",
                                    n as f64,
                                    span
                                );
                                let expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::Float(n as f64), span);
                                return self.parse_postfix(expr, start);
                            }
                            TokenType::Small => {
                                if self.match_type(&[TokenType::Float]) {
                                    let expr = match i32::try_from(n) {
                                        Ok(f) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::SFloat(f as f32), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::Int]) {
                                    let expr = match i32::try_from(n) {
                                        Ok(f) => {
                                            self.ast_arena.alloc_expr(ExpressionKind::SInt(f), span)
                                        }
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::UInt]) {
                                    let expr = match u32::try_from(n) {
                                        Ok(u) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::SUInt(u), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'small'", span));
                            }

                            TokenType::Big => {
                                if self.match_type(&[TokenType::Byte]) {
                                    let expr = match u16::try_from(n) {
                                        Ok(b) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::BByte(b), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::SByte]) {
                                    let expr = match i16::try_from(n) {
                                        Ok(b) => self
                                            .ast_arena
                                            .alloc_expr(ExpressionKind::BSByte(b), span),
                                        Err(e) => {
                                            return Err(self.err(e.to_string(), span));
                                        }
                                    };
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'big'", span));
                            }

                            TokenType::Byte => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Byte expr (from number, cast to byte): {} @ {:?}",
                                    n as u8,
                                    span
                                );
                                let expr = match u8::try_from(n) {
                                    Ok(b) => {
                                        self.ast_arena.alloc_expr(ExpressionKind::Byte(b), span)
                                    }
                                    Err(e) => {
                                        return Err(self.err(e.to_string(), span));
                                    }
                                };
                                return self.parse_postfix(expr, start);
                            }
                            TokenType::SByte => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc SByte expr (from number, cast to sbyte): {} @ {:?}",
                                    n as i8,
                                    span
                                );
                                let expr = match i8::try_from(n) {
                                    Ok(b) => {
                                        self.ast_arena.alloc_expr(ExpressionKind::SByte(b), span)
                                    }
                                    Err(e) => {
                                        return Err(self.err(e.to_string(), span));
                                    }
                                };
                                return self.parse_postfix(expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float/uint types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    // `as <alias>`: no constant folding; regular
                    // checked cast over an integer value node.
                    if let TokenType::Identifier(alias) = self.peek() {
                        if let Some(target) =
                            self.ast_arena.type_aliases.get(&alias).cloned()
                        {
                            self.advance();
                            self.ast_arena
                                .alias_uses
                                .push((alias, self.previous_span()));
                            let Ok(as_i64) = i64::try_from(n) else {
                                return Err(self.err(
                                    format!("value {} is out of range for int", n),
                                    span,
                                ));
                            };
                            let value = self
                                .ast_arena
                                .alloc_expr(ExpressionKind::Integer(as_i64), span);
                            let cast_span = start.join(self.previous_span());
                            let expr = self.ast_arena.alloc_expr(
                                ExpressionKind::Cast {
                                    value,
                                    target_type: target,
                                },
                                cast_span,
                            );
                            return self.parse_postfix(expr, start);
                        }
                    }
                    return Err(self.err("expected type after `as`", self.previous_span()));
                }
                // ---- cast end ----

                // no cast logic - defaults to `int`, so the same i64 bound applies
                let Ok(as_i64) = i64::try_from(n) else {
                    return Err(self.err(
                        format!(
                            "value {} is out of range for int ({}..={}) - use `as uint`",
                            n,
                            i64::MIN,
                            i64::MAX
                        ),
                        span,
                    ));
                };
                #[cfg(feature = "debug")]
                log::trace!(
                    "alloc Integer expr (plain number literal): {} @ {:?}",
                    as_i64,
                    span
                );
                let expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::Integer(as_i64), span);
                return self.parse_postfix(expr, start);
            }
        }

        // --- float ---
        if self.match_type(&[TokenType::FloatLiteral(0.0)]) {
            #[cfg(feature = "debug")]
            log::debug!("found number <float>");
            let span = self.previous_span();
            if let TokenType::FloatLiteral(f) = self.previous() {
                // ---- cast start ----
                if self.match_type(&[TokenType::As]) {
                    // from float to T
                    if self.match_type(&[
                        TokenType::Int,
                        TokenType::Byte,
                        TokenType::Float,
                        TokenType::UInt,
                        TokenType::Small,
                        TokenType::Big,
                        TokenType::SByte,
                    ]) {
                        match self.previous() {
                            TokenType::Int => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Integer expr (from float, cast to int): {} @ {:?}",
                                    f as i64,
                                    span
                                );
                                let float_expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::Integer(f as i64), span);
                                return self.parse_postfix(float_expr, start);
                            }

                            TokenType::Small => {
                                if self.match_type(&[TokenType::Float]) {
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc SFloat expr (from float, cast to small float): {} @ {:?}",
                                        f as f32,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::SFloat(f as f32), span);
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::Int]) {
                                    if !((i32::MIN as f64)..=(i32::MAX as f64)).contains(&f) {
                                        return Err(self.err(
                                            format!("value {} is out of range for small int", f),
                                            span,
                                        ));
                                    }
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc SInt expr (from float, cast to small int): {} @ {:?}",
                                        f as i32,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::SInt(f as i32), span);
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::UInt]) {
                                    if f < 0.0 || f > u32::MAX as f64 {
                                        return Err(self.err(
                                            format!("value {} is out of range for small uint", f),
                                            span,
                                        ));
                                    }
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc SUInt expr (from float, cast to small uint): {} @ {:?}",
                                        f as u32,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::SUInt(f as u32), span);
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'small'", span));
                            }

                            TokenType::Big => {
                                if self.match_type(&[TokenType::Byte]) {
                                    if !(0.0..=(u16::MAX as f64)).contains(&f) {
                                        return Err(self.err(
                                            format!("value {} is out of range for big byte", f),
                                            span,
                                        ));
                                    }
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc BByte expr (from float, cast to big byte): {} @ {:?}",
                                        f as u16,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::BByte(f as u16), span);
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::SByte]) {
                                    if !((i16::MIN as f64)..=(i16::MAX as f64)).contains(&f) {
                                        return Err(self.err(
                                            format!("value {} is out of range for big sbyte", f),
                                            span,
                                        ));
                                    }
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc BSByte expr (from float, cast to big sbyte): {} @ {:?}",
                                        f as i16,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::BSByte(f as i16), span);
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'big'", span));
                            }

                            TokenType::SByte => {
                                if !((i8::MIN as f64)..=(i8::MAX as f64)).contains(&f) {
                                    return Err(self
                                        .err(format!("value {} is too large for sbyte", f), span));
                                }
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc SByte expr (from float, cast to sbyte): {} @ {:?}",
                                    f as i8,
                                    span
                                );
                                let expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::SByte(f as i8), span);
                                return self.parse_postfix(expr, start);
                            }

                            TokenType::UInt => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc UInt expr (from float, cast to uint): {} @ {:?}",
                                    f as u64,
                                    span
                                );
                                if f < 0.0 {
                                    return Err(self.err(
                                        format!("value {} is negative, cannot cast to uint", f),
                                        span,
                                    ));
                                }
                                let float_expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::UInt(f as u64), span);
                                return self.parse_postfix(float_expr, start);
                            }

                            TokenType::Float => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Float expr (from float, cast to float): {} @ {:?}",
                                    f,
                                    span
                                );
                                let float_expr =
                                    self.ast_arena.alloc_expr(ExpressionKind::Float(f), span);
                                return self.parse_postfix(float_expr, start);
                            }

                            TokenType::Byte => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Byte expr (from float, cast to byte): {} @ {:?}",
                                    f as u8,
                                    span
                                );
                                if !(0.0..=255.0).contains(&f) {
                                    return Err(self
                                        .err(format!("value {} is too large for byte", f), span));
                                }
                                let float_expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::Byte(f as u8), span);
                                return self.parse_postfix(float_expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float/uint types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    // `as <alias>`: regular checked cast over a float node.
                    if let TokenType::Identifier(alias) = self.peek() {
                        if let Some(target) =
                            self.ast_arena.type_aliases.get(&alias).cloned()
                        {
                            self.advance();
                            self.ast_arena
                                .alias_uses
                                .push((alias, self.previous_span()));
                            let value = self
                                .ast_arena
                                .alloc_expr(ExpressionKind::Float(f), span);
                            let cast_span = start.join(self.previous_span());
                            let expr = self.ast_arena.alloc_expr(
                                ExpressionKind::Cast {
                                    value,
                                    target_type: target,
                                },
                                cast_span,
                            );
                            return self.parse_postfix(expr, start);
                        }
                    }
                    return Err(self.err("expected type after `as`", span));
                }
                // ---- cast end ----

                // no cast logic
                #[cfg(feature = "debug")]
                log::trace!("alloc Float expr (plain float literal): {} @ {:?}", f, span);
                let expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::Float(f), self.previous_span());
                return self.parse_postfix(expr, start);
            }
        }

        // --- byte ---
        if self.match_type(&[TokenType::ByteLiteral(0)]) {
            #[cfg(feature = "debug")]
            log::debug!("found number <byte>");
            let span = self.previous_span();
            if let TokenType::ByteLiteral(b) = self.previous() {
                // ---- cast start ----
                if self.match_type(&[TokenType::As]) {
                    // from byte to T
                    if self.match_type(&[
                        TokenType::Int,
                        TokenType::Byte,
                        TokenType::Float,
                        TokenType::UInt,
                        TokenType::Small,
                        TokenType::Big,
                        TokenType::SByte,
                    ]) {
                        match self.previous() {
                            TokenType::Int => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Integer expr (from byte, cast to int): {} @ {:?}",
                                    b as i64,
                                    span
                                );
                                let byte_expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::Integer(b as i64), span);
                                return self.parse_postfix(byte_expr, start);
                            }

                            TokenType::Small => {
                                if self.match_type(&[TokenType::Float]) {
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc SFloat expr (from byte, cast to small float): {} @ {:?}",
                                        b as f32,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::SFloat(b as f32), span);
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::Int]) {
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc SInt expr (from byte, cast to small int): {} @ {:?}",
                                        b as i32,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::SInt(b as i32), span);
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::UInt]) {
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc SUInt expr (from byte, cast to small uint): {} @ {:?}",
                                        b as u32,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::SUInt(b as u32), span);
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'small'", span));
                            }

                            TokenType::Big => {
                                if self.match_type(&[TokenType::Byte]) {
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc BByte expr (from byte, cast to big byte): {} @ {:?}",
                                        b as u16,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::BByte(b as u16), span);
                                    return self.parse_postfix(expr, start);
                                }
                                if self.match_type(&[TokenType::SByte]) {
                                    #[cfg(feature = "debug")]
                                    log::trace!(
                                        "alloc BSByte expr (from byte, cast to big sbyte): {} @ {:?}",
                                        b as i16,
                                        span
                                    );
                                    let expr = self
                                        .ast_arena
                                        .alloc_expr(ExpressionKind::BSByte(b as i16), span);
                                    return self.parse_postfix(expr, start);
                                }
                                return Err(self.err("expected valid type after 'big'", span));
                            }

                            TokenType::SByte => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc SByte expr (from byte, cast to sbyte): {} @ {:?}",
                                    b,
                                    span
                                );
                                let expr = match i8::try_from(b) {
                                    Ok(sb) => {
                                        self.ast_arena.alloc_expr(ExpressionKind::SByte(sb), span)
                                    }
                                    Err(e) => {
                                        return Err(self.err(e.to_string(), span));
                                    }
                                };
                                return self.parse_postfix(expr, start);
                            }

                            TokenType::UInt => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc UInt expr (from byte, cast to uint): {} @ {:?}",
                                    b as u64,
                                    span
                                );
                                // a byte is 0..=255, always a valid uint - no bounds check needed
                                let byte_expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::UInt(b as u64), span);
                                return self.parse_postfix(byte_expr, start);
                            }

                            TokenType::Float => {
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Float expr (from byte, cast to float): {} @ {:?}",
                                    b as f64,
                                    span
                                );
                                let byte_expr = self
                                    .ast_arena
                                    .alloc_expr(ExpressionKind::Float(b as f64), span);
                                return self.parse_postfix(byte_expr, start);
                            }

                            TokenType::Byte => {
                                // while it shouldn't be possible normally
                                // keeping it as fallback for safety
                                #[cfg(feature = "debug")]
                                log::trace!(
                                    "alloc Byte expr (from byte, cast to byte): {} @ {:?}",
                                    b,
                                    span
                                );
                                if !(0..=255).contains(&b) {
                                    return Err(self
                                        .err(format!("value {} is too large for byte", b), span));
                                }
                                let byte_expr =
                                    self.ast_arena.alloc_expr(ExpressionKind::Byte(b), span);
                                return self.parse_postfix(byte_expr, start);
                            }

                            other => {
                                return Err(self.err(
                                    format!("expected int/byte/float/uint types found {:?}", other),
                                    span,
                                ));
                            }
                        }
                    }
                    // `as <alias>`: regular checked cast over a byte node.
                    if let TokenType::Identifier(alias) = self.peek() {
                        if let Some(target) =
                            self.ast_arena.type_aliases.get(&alias).cloned()
                        {
                            self.advance();
                            self.ast_arena
                                .alias_uses
                                .push((alias, self.previous_span()));
                            let value = self
                                .ast_arena
                                .alloc_expr(ExpressionKind::Byte(b), span);
                            let cast_span = start.join(self.previous_span());
                            let expr = self.ast_arena.alloc_expr(
                                ExpressionKind::Cast {
                                    value,
                                    target_type: target,
                                },
                                cast_span,
                            );
                            return self.parse_postfix(expr, start);
                        }
                    }
                    return Err(self.err("expected type after `as`", self.previous_span()));
                }
                // ---- cast end ----

                // no cast logic
                #[cfg(feature = "debug")]
                log::trace!("alloc Byte expr (plain byte literal): {} @ {:?}", b, span);
                let expr = self.ast_arena.alloc_expr(ExpressionKind::Byte(b), span);
                return self.parse_postfix(expr, start);
            }
        }

        // --- suffixed integer types ---
        if self.match_type(&[TokenType::SignedByteLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::SignedByteLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::SByte(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        if self.match_type(&[TokenType::BigByteLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::BigByteLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::BByte(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        if self.match_type(&[TokenType::BigSignedByteLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::BigSignedByteLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::BSByte(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        if self.match_type(&[TokenType::SmallIntLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::SmallIntLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::SInt(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        if self.match_type(&[TokenType::SmallUIntLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::SmallUIntLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::SUInt(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        if self.match_type(&[TokenType::SmallFloatLiteral(0.0)]) {
            let span = self.previous_span();
            if let TokenType::SmallFloatLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::SFloat(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        if self.match_type(&[TokenType::UIntLiteral(0)]) {
            let span = self.previous_span();
            if let TokenType::UIntLiteral(v) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::UInt(v), span);
                return self.parse_postfix(expr, start);
            }
        }
        // ---- numbers end ----

        // --- string ---
        if self.match_type(&[TokenType::StringLiteral(String::new())]) {
            #[cfg(feature = "debug")]
            log::debug!("found string");
            let span = self.previous_span();
            if let TokenType::StringLiteral(s) = self.previous() {
                #[cfg(feature = "debug")]
                log::trace!("alloc String expr: {:?} @ {:?}", s, span);
                let expr = self.ast_arena.alloc_expr(ExpressionKind::String(s), span);
                return self.parse_postfix(expr, start);
            }
        }

        // --- character ---
        if matches!(
            self.tokens[self.current].token,
            TokenType::CharacterLiteral(_)
        ) {
            self.advance();
            #[cfg(feature = "debug")]
            log::debug!("found character");
            let span = self.previous_span();
            if let TokenType::CharacterLiteral(c) = self.previous() {
                #[cfg(feature = "debug")]
                log::trace!("alloc Character expr: {:?} @ {:?}", c, span);
                let expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::Character(c), span);
                return self.parse_postfix(expr, start);
            }
        }

        // --- boolean ---
        if self.match_type(&[TokenType::BoolLiteral(false)]) {
            let span = self.previous_span();
            if let TokenType::BoolLiteral(b) = self.previous() {
                let expr = self.ast_arena.alloc_expr(ExpressionKind::Bool(b), span);
                #[cfg(feature = "debug")]
                log::trace!("alloc Bool expr: {} @ {:?}", b, span);
                return self.parse_postfix(expr, start);
            }
        }

        // --- error ---
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
                .ast_arena
                .alloc_expr(ExpressionKind::ErrorLiteral(inner), span);
            #[cfg(feature = "debug")]
            log::trace!("alloc ErrorLiteral expr: inner={:?} @ {:?}", inner, span);
            return self.parse_postfix(expr, start);
        }

        // ---- result start ----
        // --- ok ---
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
            #[cfg(feature = "debug")]
            log::trace!("alloc OkLiteral expr: inner={:?} @ {:?}", inner, span);
            let ok_expr = self
                .ast_arena
                .alloc_expr(ExpressionKind::OkLiteral(inner), span);
            return self.parse_postfix(ok_expr, start);
        }

        // --- err ---
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
            #[cfg(feature = "debug")]
            log::trace!("alloc ErrLiteral expr: inner={:?} @ {:?}", inner, span);
            let err_expr = self
                .ast_arena
                .alloc_expr(ExpressionKind::ErrLiteral(inner), span);
            return self.parse_postfix(err_expr, start);
        }
        // ---- result end ----

        // --- null ---
        if self.match_type(&[TokenType::Null]) {
            let span = self.previous_span();
            let expr = self.ast_arena.alloc_expr(ExpressionKind::Null, span);
            #[cfg(feature = "debug")]
            log::trace!("alloc Null expr @ {:?}", span);
            return self.parse_postfix(expr, start);
        }

        // --- group ---
        if self.match_type(&[TokenType::LeftParen]) {
            #[cfg(feature = "debug")]
            log::debug!("found group start");
            while self.match_type(&[TokenType::Newline]) {}
            let first = self.parse_expression()?;

            if self.match_type(&[TokenType::Comma]) {
                // --- tuple literal ---
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
                #[cfg(feature = "debug")]
                log::trace!(
                    "alloc TupleLiteral expr: items={} @ {:?}",
                    items.len(),
                    span
                );

                let expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::TupleLiteral(items), span);
                return self.parse_postfix(expr, start);
            }

            // normal group
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightParen]) {
                return Err(self.err("expected `)` after grouped expression", self.peek_span()));
            }

            let span = start.join(self.previous_span());
            let expr = self
                .ast_arena
                .alloc_expr(ExpressionKind::Grouping(first), span);
            #[cfg(feature = "debug")]
            log::trace!("alloc Grouping expr: inner={:?} @ {:?}", first, span);
            return self.parse_postfix(expr, start);
        }

        // --- function/lambda ---
        if self.match_type(&[TokenType::Fn]) {
            let lambda_start = self.previous_span();
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::LeftParen]) {
                return Err(self.err("expected `(` after `fn`", self.peek_span()));
            }

            let mut params: Vec<rl_ast::statements::Param> = Vec::new();
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
                params.push(rl_ast::statements::Param {
                    param_name,
                    param_type,
                });
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::Comma]) {
                    while self.match_type(&[TokenType::Newline]) {}
                    if !self.match_type(&[TokenType::RightParen]) {
                        return Err(self.err("expected `)` after lambda parameters", self.peek_span()));
                    }
                    break;
                }
                while self.match_type(&[TokenType::Newline]) {}
            }

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
            #[cfg(feature = "debug")]
            log::trace!(
                "alloc Lambda expr: params={} return_type={:?} body_stmts={} @ {:?}",
                params.len(),
                return_type,
                body.len(),
                span
            );
            let expr = self.ast_arena.alloc_expr(
                ExpressionKind::Lambda {
                    params,
                    return_type,
                    body,
                },
                span,
            );
            return self.parse_postfix(expr, start);
        }

        // --- not valid expression ---
        Err(self.err("expected expression", self.peek_span()))
    }
}
