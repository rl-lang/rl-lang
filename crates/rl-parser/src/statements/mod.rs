//! Top-level statement dispatcher and block parser.
//!
//! [`parse_statement_to_ast`] is the main entry point called by the top-level
//! parse loop in [`Parser::parse`]. It peeks at the current token and routes
//! to the appropriate sub-parser. Anything that does not start with a known
//! keyword is parsed as a bare expression statement.
//!
//! [`parse_statement_to_ast`]: Parser::parse_statement_to_ast
//! [`Parser::parse`]: crate::parser::parser_logic::Parser::parse

mod const_declaration;
mod for_statement;
mod function_declaration;
mod if_statement;
mod impl_block;
mod import_statement;
mod infer_declaration;
mod match_statement;
mod program_attribute;
mod record_declaration;
mod tag_declaration;
mod variable_declaration;
mod while_statement;

use crate::parser_logic::Parser;
use rl_ast::{
    nodes::ExpressionKind,
    statements::{FunctionAttribute, ItemAttribute, Lint, Statement, StatementKind},
};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Dispatches the current token to the appropriate statement sub-parser.
    ///
    /// | Token | Action |
    /// |---|---|
    /// | `Newline` | skip - emits a no-op `Expression(0)` placeholder |
    /// | `get` | [`parse_import`] |
    /// | `dec` | [`parse_variable_declaration`] |
    /// | `dec` (infer) | [`parse_let_declaration`] |
    /// | `CONST` | [`parse_const_declaration`] |
    /// | `while` | [`parse_while`] |
    /// | `for` | [`parse_for`] |
    /// | `if` | [`parse_if`] |
    /// | `fn` | [`parse_function`] |
    /// | `!#` | [`parse_entry_attribute`] |
    /// | `return` | inline - parses optional return value |
    /// | `break` | inline - emits [`StatementKind::Break`] |
    /// | `continue` | inline - emits [`StatementKind::Continue`] |
    /// | anything else | [`parse_expression`] wrapped in [`StatementKind::Expression`] |
    ///
    /// [`parse_import`]: Parser::parse_import
    /// [`parse_variable_declaration`]: Parser::parse_variable_declartion
    /// [`parse_infer_declaration`]: Parser::parse_infer_declaration
    /// [`parse_const_declaration`]: Parser::parse_const_declartion
    /// [`parse_while`]: Parser::parse_while
    /// [`parse_for`]: Parser::parse_for
    /// [`parse_if`]: Parser::parse_if
    /// [`parse_function`]: Parser::parse_function
    /// [`parse_entry_attribute`]: Parser::parse_entry_attribute
    /// [`parse_expression`]: Parser::parse_expression
    pub fn parse_statement_to_ast(&mut self) -> Result<Statement, Error> {
        let start = self.peek_span();
        let result = match self.peek() {
            TokenType::Newline => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found newline while parsing... skipping");
                let span = self.previous_span();
                Statement::new(
                    StatementKind::Expression(
                        self.ast_arena.alloc_expr(ExpressionKind::Integer(0), span),
                    ),
                    span,
                )
            }

            TokenType::Get => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `get` for import while parsing");
                while self.match_type(&[TokenType::Newline]) {}
                self.parse_import(start)?
            }
            TokenType::Dec => {
                self.advance();

                let is_inferred = if let TokenType::Identifier(name) = self.peek() {
                    !self.record_names.contains(&name) && !self.tag_names.contains(&name)
                } else {
                    false
                };

                if is_inferred {
                    #[cfg(feature = "debug")]
                    log::info!("found `dec` for inferred variable while parsing");
                    self.parse_infer_declaration(start)?
                } else {
                    #[cfg(feature = "debug")]
                    log::info!("found `dec` for variable (record|tag) while parsing");
                    self.parse_variable_declartion(start)?
                }
            }
            TokenType::Const => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `declaration` for constant while parsing");
                self.parse_const_declartion(start)?
            }
            TokenType::While => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `while` while parsing");
                self.parse_while(start)?
            }
            TokenType::Loop => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `loop` while parsing");
                self.parse_loop(start)?
            }
            TokenType::For => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `for` while parsing");
                self.parse_for(start)?
            }
            TokenType::If => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `if` while parsing");
                self.parse_if(start)?
            }

            TokenType::Fn => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found 'fn' while parsing");
                self.parse_function(start, None, Vec::new())?
            }

            TokenType::BangHash => {
                self.advance();
                self.parse_entry_attribute(start)?
            }
            TokenType::Return => {
                self.advance();
                // parse the return value only when one is present on the same line
                let expr = if !matches!(self.peek(), TokenType::Newline)
                    && !matches!(self.peek(), TokenType::RightBrace)
                    && !self.is_at_end()
                {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                let span = start.join(self.previous_span());
                Statement::new(StatementKind::Return(expr), span)
            }

            TokenType::Break => {
                self.advance();
                let span = start.join(self.previous_span());
                Statement::new(StatementKind::Break, span)
            }

            TokenType::Continue => {
                self.advance();
                let span = start.join(self.previous_span());
                Statement::new(StatementKind::Continue, span)
            }

            TokenType::Match => {
                self.advance();
                self.parse_match(start)?
            }

            TokenType::Record => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `record` while parsing");
                self.parse_record_declaration(start)?
            }

            TokenType::Tag => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `tag` while parsing");
                self.parse_tag_declaration(start)?
            }

            TokenType::Impl => {
                self.advance();
                #[cfg(feature = "debug")]
                log::info!("found `impl` while parsing");
                self.parse_impl_block(start)?
            }

            _ => {
                #[cfg(feature = "debug")]
                log::info!("parsing the current tokens as expression");
                let expr = self.parse_expression()?;
                let span = self.ast_arena.exprs.get(expr).span;
                Statement::new(StatementKind::Expression(expr), span)
            }
        };

        self.match_type(&[TokenType::Semicolon]);
        Ok(result)
    }

    /// Parses a brace-delimited block `{ stmts* }` into a [`Vec<Statement>`].
    ///
    /// Consumes the opening `{`, then repeatedly calls [`parse_statement_to_ast`]
    /// until `}` or [`TokenType::Eof`] is reached. Blank lines (newlines) inside
    /// the block are skipped.
    ///
    /// # Errors
    /// Returns an error if the opening `{` is missing.
    ///
    /// [`parse_statement_to_ast`]: Parser::parse_statement_to_ast
    pub fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        if !self.match_type(&[TokenType::LeftBrace]) {
            return Err(self.err("expected `{`", self.peek_span()));
        }
        let mut statements = Vec::new();

        #[cfg(feature = "debug")]
        log::info!("parsing body into statements");
        while !self.match_type(&[TokenType::RightBrace, TokenType::Eof]) {
            if matches!(self.peek(), TokenType::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement_to_ast()?);
        }
        Ok(statements)
    }

    /// Parses a `!#[entry]` attribute and the `fn` declaration that follows it.
    ///
    /// The `!#` token has already been consumed by [`parse_statement_to_ast`]
    /// before this is called. Expects `[entry]` then a function declaration,
    /// which is forwarded to [`parse_function`] with `is_entry = true`.
    ///
    /// # Errors
    /// Returns an error if `[`, the `entry` identifier, `]`, or `fn` are missing
    /// or in the wrong order.
    ///
    /// [`parse_function`]: Parser::parse_function
    fn parse_entry_attribute(&mut self, start: Span) -> Result<Statement, Error> {
        if !self.match_type(&[TokenType::LeftBracket]) {
            return Err(self.err("expected `[` after `!#`", self.peek_span()));
        }

        while self.match_type(&[TokenType::Newline]) {}

        let mut function_attr: Option<FunctionAttribute> = None;
        let mut item_attrs: Vec<ItemAttribute> = Vec::new();

        loop {
            match self.peek() {
                // existing function lifecycle attributes
                TokenType::Identifier(name) if name == "entry" => {
                    self.advance();
                    if self.check(&TokenType::Assign) {
                        return Err(self.err("`!#[entry]` does not take a priority", self.peek_span()));
                    }
                    function_attr = Some(FunctionAttribute::Entry);
                }
                TokenType::Identifier(name) if name == "init" => {
                    self.advance();
                    function_attr = Some(FunctionAttribute::Init(self.parse_optional_priority()?));
                }
                TokenType::Identifier(name) if name == "final" => {
                    self.advance();
                    function_attr = Some(FunctionAttribute::Final(self.parse_optional_priority()?));
                }
                TokenType::Identifier(name) if name == "test" => {
                    self.advance();
                    if self.check(&TokenType::Assign) {
                        return Err(self.err("`!#[test]` does not take a priority", self.peek_span()));
                    }
                    function_attr = Some(FunctionAttribute::Test);
                }
                // new: allow(unused, deprecated, ...)
                TokenType::Identifier(name) if name == "allow" => {
                    self.advance();
                    let lints = self.parse_lint_list()?;
                    item_attrs.push(ItemAttribute::Allow(lints));
                }
                // new: deprecated or deprecated("msg")
                TokenType::Identifier(name) if name == "deprecated" => {
                    self.advance();
                    let msg = self.parse_optional_string_arg()?.or_else(|| Some(String::new()));
                    item_attrs.push(ItemAttribute::Deprecated(msg));
                }
                // custom markers from `#![define(name)]`: bare or with
                // string args. Anything else is still a typo error.
                TokenType::Identifier(name) => {
                    let name = name.clone();
                    if !self.custom_attr_defined(&name) {
                        return Err(self.err("expected valid attribute", self.peek_span()));
                    }
                    self.advance();
                    let args = self.parse_string_list_arg()?;
                    item_attrs.push(ItemAttribute::Custom { name, args });
                }
                _ => return Err(self.err("expected valid attribute", self.peek_span())),
            }

            while self.match_type(&[TokenType::Newline]) {}
            if self.match_type(&[TokenType::Comma]) {
                while self.match_type(&[TokenType::Newline]) {}
                continue;
            }
            break;
        }

        while self.match_type(&[TokenType::Newline]) {}

        if !self.match_type(&[TokenType::RightBracket]) {
            return Err(self.err("expected `]` after attribute", self.peek_span()));
        }

        while self.match_type(&[TokenType::Newline]) {}

        // dispatch on whichever item follows
        match self.peek() {
            TokenType::Fn => {
                self.advance();
                self.parse_function(start, function_attr, item_attrs)
            }
            TokenType::Dec => {
                self.advance();
                let is_inferred = if let TokenType::Identifier(name) = self.peek() {
                    !self.record_names.contains(&name) && !self.tag_names.contains(&name)
                } else {
                    false
                };
                let mut stmt = if is_inferred {
                    self.parse_infer_declaration(start)?
                } else {
                    self.parse_variable_declartion(start)?
                };
                Self::inject_item_attributes(&mut stmt, item_attrs);
                Ok(stmt)
            }
            TokenType::Const => {
                self.advance();
                let mut stmt = self.parse_const_declartion(start)?;
                Self::inject_item_attributes(&mut stmt, item_attrs);
                Ok(stmt)
            }
            _ => Err(self.err(
                "expected fn, dec, or const after `!#[...]`",
                self.peek_span(),
            )),
        }
    }

    /// Injects `item_attributes` into the returned statement's kind.
    fn inject_item_attributes(stmt: &mut Statement, attrs: Vec<ItemAttribute>) {
        if attrs.is_empty() {
            return;
        }
        match &mut stmt.kind {
            StatementKind::VariableDeclaration { item_attributes, .. } => {
                *item_attributes = attrs;
            }
            StatementKind::ConstantDeclaration { item_attributes, .. } => {
                *item_attributes = attrs;
            }
            StatementKind::FunctionDeclaration { item_attributes, .. } => {
                *item_attributes = attrs;
            }
            _ => {}
        }
    }

    /// Parses `(ident, ident, ...)` for `!#[allow(...)]`, validating each name
    /// against the known `Lint` set at parse time.
    fn parse_lint_list(&mut self) -> Result<Vec<Lint>, Error> {
        if !self.match_type(&[TokenType::LeftParen]) {
            return Err(self.err("expected `(` after `allow`", self.peek_span()));
        }
        let mut lints = Vec::new();
        loop {
            match self.peek() {
                TokenType::Identifier(name) if name == "unused" => {
                    self.advance();
                    lints.push(Lint::Unused);
                }
                TokenType::Identifier(name) if name == "deprecated" => {
                    self.advance();
                    lints.push(Lint::Deprecated);
                }
                TokenType::Identifier(name) => {
                    return Err(self.err(format!("unknown lint `{}`", name), self.peek_span()));
                }
                _ => return Err(self.err("expected a lint name", self.peek_span())),
            }
            if self.match_type(&[TokenType::Comma]) {
                while self.match_type(&[TokenType::Newline]) {}
                continue;
            }
            break;
        }
        if !self.match_type(&[TokenType::RightParen]) {
            return Err(self.err("expected `)` after lint list", self.peek_span()));
        }
        Ok(lints)
    }

    /// Parses `("literal")` or nothing, for `!#[deprecated]` / `!#[deprecated("msg")]`.
    fn parse_optional_string_arg(&mut self) -> Result<Option<String>, Error> {
        if !self.match_type(&[TokenType::LeftParen]) {
            return Ok(None);
        }
        let msg = match self.peek() {
            TokenType::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => return Err(self.err("expected a string literal", self.peek_span())),
        };
        if !self.match_type(&[TokenType::RightParen]) {
            return Err(self.err("expected `)`", self.peek_span()));
        }
        Ok(Some(msg))
    }

    /// Parses an optional `=n` priority suffix for `!#[init=n]` / `!#[final=n]`.
    ///
    /// Returns `Some(n)` when the next token is `=` followed by a number
    /// literal, otherwise `None` (the unnumbered form, which runs last).
    ///
    /// # Errors
    /// Returns an error if `=` is present but not followed by a number.
    fn parse_optional_priority(&mut self) -> Result<Option<u32>, Error> {
        if !self.match_type(&[TokenType::Assign]) {
            return Ok(None);
        }

        match self.peek() {
            TokenType::NumberLiteral(value) => {
                let priority = u32::try_from(value)
                    .map_err(|_| self.err("priority is out of range", self.peek_span()))?;
                self.advance();
                Ok(Some(priority))
            }
            _ => Err(self.err("expected a number after `=`", self.peek_span())),
        }
    }

    /// True when `name` was declared by a `#![define(name)]` seen so far.
    /// Defines are declare-before-use like values: a use above its
    /// `#![define]` is an unknown attribute, not a forward reference.
    fn custom_attr_defined(&self, name: &str) -> bool {
        use rl_ast::statements::ProgramAttribute;
        self.ast_arena.program_attributes.iter().any(|attr| {
            matches!(attr, ProgramAttribute::Define { name: prev } if prev == name)
        })
    }

    /// Parses `("a", "b")` or nothing, for custom `!#[name]` markers.
    fn parse_string_list_arg(&mut self) -> Result<Vec<String>, Error> {
        if !self.match_type(&[TokenType::LeftParen]) {
            return Ok(Vec::new());
        }
        let mut args = Vec::new();
        loop {
            while self.match_type(&[TokenType::Newline]) {}
            match self.peek() {
                TokenType::StringLiteral(s) => {
                    args.push(s.clone());
                    self.advance();
                }
                _ => return Err(self.err("expected a string literal", self.peek_span())),
            }
            while self.match_type(&[TokenType::Newline]) {}
            if self.match_type(&[TokenType::Comma]) {
                continue;
            }
            break;
        }
        if !self.match_type(&[TokenType::RightParen]) {
            return Err(self.err("expected `)`", self.peek_span()));
        }
        Ok(args)
    }
}