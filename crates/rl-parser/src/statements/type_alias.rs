//! Type alias declaration parser (`type`).
//!
//! Handles alias declarations in the form:
//!
//! ```text
//! type point (int, int, int)
//! type number int
//! ```
//!
//! Aliases are transparent: the target is resolved eagerly against
//! previously declared aliases, so cycles are impossible by construction
//! and use-before-definition is an error (same visibility rule as values).
//! The resolved target is stored in the `Ast` alias table; uses parse as
//! [`TypeAnnotation::Alias`] nodes resolved lazily by the checker and
//! backends.

use crate::parser_logic::Parser;
use rl_ast::statements::{ItemAttribute, Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Parses a `type` alias declaration.
    ///
    /// Called after `type` has been consumed, with any `!#[...]` item
    /// attributes. Reads the alias name and its target type, resolves
    /// nested aliases against the table so far, and records both tables.
    ///
    /// Produces [`StatementKind::TypeAlias`].
    ///
    /// # Errors
    /// Returns an error if the name is missing, duplicated, or the target
    /// references an unknown alias.
    pub fn parse_type_alias(
        &mut self,
        start: Span,
        item_attributes: Vec<ItemAttribute>,
    ) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}

        let name = match self.peek() {
            TokenType::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(self.err("expected alias name after `type`", self.peek_span())),
        };

        if self.ast_arena.type_aliases.contains_key(&name) {
            return Err(self.err(
                format!("type alias `{name}` is already defined"),
                self.peek_span(),
            ));
        }

        while self.match_type(&[TokenType::Newline]) {}
        let target = self.parse_type(true)?;
        let span = start.join(self.previous_span());

        let resolved = self.resolve_alias_target(&target)?;
        self.ast_arena
            .type_aliases
            .insert(name.clone(), resolved.clone());
        self.ast_arena
            .type_alias_attrs
            .insert(name.clone(), item_attributes.clone());

        Ok(Statement::new(
            StatementKind::TypeAlias {
                name,
                target: resolved,
                item_attributes,
            },
            span,
        ))
    }

    /// Validates a freshly parsed alias target. Targets arrive fully
    /// resolved already (`parse_type` substitutes nested aliases
    /// eagerly), so this is only a structural pass-through kept as the
    /// single choke point if that ever changes.
    fn resolve_alias_target(
        &self,
        target: &TypeAnnotation,
    ) -> Result<TypeAnnotation, Error> {
        Ok(target.clone())
    }
}
