//! Inferred variable declaration parser.
//!
//! ```text
//! dec x = 42
//! dec name = "hi"
//! dec xs = [1, 2, 3]
//! ```
//!
//! `dec` now produces a [`StatementKind::VariableDeclaration`] with
//! [`TypeAnnotation::Infer`] if an explicit type is missing. Downstream passes
//! (the checker and the interpreter) replace `Infer` with the initialiser
//! expression's actual type.

use crate::parser_logic::Parser;
use rl_ast::statements::{Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Parses a infer variable declaration.
    ///
    /// Called after `dec` has been consumed and confirmed the next token is identifier that isn't record or tag.
    /// Expects `name = expr` and
    /// produces [`StatementKind::VariableDeclaration`] with
    /// `type_annotation: TypeAnnotation::Infer`.
    ///
    /// # Errors
    /// Returns an error if the name, `=`, or initialiser expression is
    /// missing or malformed.
    pub fn parse_infer_declaration(&mut self, start: Span) -> Result<Statement, Error> {
        #[cfg(feature = "debug")]
        log::debug!("parsing inferred `dec` declaration");

        while self.match_type(&[TokenType::Newline]) {}
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected name after `dec`", self.peek_span())),
        };

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::Assign]) {
            return Err(self.err("expected `=` after name", self.peek_span()));
        }

        while self.match_type(&[TokenType::Newline]) {}
        let value = self.parse_expression()?;
        let value_id = self.ast_arena.exprs.get(value);
        let span = start.join(value_id.span);

        Ok(Statement::new(
            StatementKind::VariableDeclaration {
                name,
                type_annotation: TypeAnnotation::Infer,
                unit_annotation: None,
                value,
            },
            span,
        ))
    }
}
