//! Precedence-climbing expression parser.
//!
//! Expressions are parsed via a hand-written recursive-descent chain that
//! encodes operator precedence through the call stack. From lowest to highest:
//!
//! ```text
//! parse_expression        (entry point, delegates to equality)
//!   |- parse_logical      (and or)
//!      |- parse_equality     (== !=)
//!          |- parse_comparison  (< <= > >= += -= *= /=)
//!               |- parse_term   (+ -)
//!                    |- parse_factor  (* /)
//!                         |- parse_unary   (! -)
//!                              |- parse_primary  (literals, identifiers, calls, lambdas, …)
//!                                   |- parse_postfix  (. method chains)
//! ```
//!
//! Every function returns an [`Expression`] node with its source [`Span`] set
//! to the full extent of the sub-expression it consumed.

mod comparsion;
mod equality;
mod factor;
mod logical;
mod postfix;
mod primary;
mod term;
mod unary;

use crate::{ast::nodes::Expression, parser::parser_logic::Parser, utils::errors::Error};

impl Parser {
    /// Entry point for expression parsing. Delegates to [`parse_equality`].
    ///
    /// [`parse_equality`]: Parser::parse_equality
    pub fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_logical()
    }
}
