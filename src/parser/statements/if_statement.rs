//! Conditional statement parser (`if` / `else if` / `else`).
//!
//! Parses the full if-chain into a tree of nested [`StatementKind::Conditional`]
//! nodes. Each `else if` branch is represented by recursion, so the AST mirrors
//! the source nesting naturally:
//!
//! ```text
//! if (a) { … } else if (b) { … } else { … }
//! ```
//! becomes:
//! ```text
//! Conditional {
//!     if_branch:   ConditionalBranch { condition: Some(a), body: […] }
//!     else_branch: Some(Conditional {
//!         if_branch:   ConditionalBranch { condition: Some(b), body: […] }
//!         else_branch: Some(ConditionalBranch { condition: None, body: […] })
//!     })
//! }
//! ```

use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses an `if` statement, including any `else if` / `else` tail.
    ///
    /// Called after `if` has been consumed. Reads:
    ///
    /// 1. The condition expression.
    /// 2. The `{`-delimited if-body via [`parse_block`].
    /// 3. An optional `else` tail - either another `if` (recursed) or a plain
    ///    `else { … }` block (condition is `None`).
    ///
    /// Blank lines between the closing `}` and `else` are skipped.
    ///
    /// Produces [`StatementKind::Conditional`] with one or two
    /// [`StatementKind::ConditionalBranch`] children.
    ///
    /// [`parse_block`]: Parser::parse_block
    pub fn parse_if(&mut self, start: Span) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}
        let if_condition = self.parse_expression()?;
        while self.match_type(&[TokenType::Newline]) {}
        let if_body = self.parse_block()?;
        let if_branch_span = start.join(self.previous_span());
        let if_branch = Statement::new(
            StatementKind::ConditionalBranch {
                condition: Some(if_condition),
                body: if_body,
                needs_scope: true,
            },
            if_branch_span,
        );

        // skip any blank lines between `}` and `else`
        while self.match_type(&[TokenType::Newline]) {}

        let else_branch = if self.peek() == TokenType::Else {
            let branch_start = self.peek_span();
            self.advance();
            if self.peek() == TokenType::If {
                // `else if` - recurse to produce a nested Conditional
                let elif_start = self.peek_span();
                self.advance();
                Some(Box::new(self.parse_if(elif_start)?))
            } else {
                // plain `else { … }` - condition is None
                let else_body = self.parse_block()?;
                let span = branch_start.join(self.previous_span());
                Some(Box::new(Statement::new(
                    StatementKind::ConditionalBranch {
                        condition: None,
                        body: else_body,
                        needs_scope: true,
                    },
                    span,
                )))
            }
        } else {
            None
        };

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::Conditional {
                if_branch: Box::new(if_branch),
                else_branch,
            },
            span,
        ))
    }
}
